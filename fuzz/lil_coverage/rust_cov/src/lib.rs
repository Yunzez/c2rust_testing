use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;

// ---------------- Basic Types ----------------

pub type LilInt = i64;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LilValue(String);

impl LilValue {
    pub fn new(s: impl Into<String>) -> Self {
        LilValue(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Default)]
pub struct LilList(Vec<LilValue>);

impl LilList {
    pub fn new() -> Self {
        LilList(Vec::new())
    }

    pub fn push(&mut self, v: LilValue) {
        self.0.push(v);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, i: usize) -> Option<&LilValue> {
        self.0.get(i)
    }

    pub fn into_vec(self) -> Vec<LilValue> {
        self.0
    }
}

// ---------------- Environment & Functions ----------------

#[derive(Clone)]
struct LilVar {
    name: String,
    value: LilValue,
    // env backref not needed in safe Rust version
}

#[derive(Clone)]
pub enum LilFunctionImpl {
    Builtin(fn(&LilInterpreter, &[LilValue]) -> LilValueOpt),
    User {
        code: LilValue,
        arg_names: LilList,
    },
}

#[derive(Clone)]
pub struct LilFunction {
    pub name: String,
    pub implementation: LilFunctionImpl,
}

#[derive(Clone, Default)]
struct LilEnv {
    parent: Option<Rc<RefCell<LilEnv>>>,
    func: Option<Rc<LilFunction>>, // current function for this env
    catcher_for: Option<LilValue>,
    vars: Vec<LilVar>,
    retval: Option<LilValue>,
    retval_set: bool,
    break_run: bool,
}

// A LilValue or "no value" (NULL in C)
pub type LilValueOpt = Option<LilValue>;

// ---------------- Callbacks ----------------

pub struct LilCallbacks {
    pub set_var: Option<Rc<dyn Fn(&LilInterpreter, &str, &mut LilValue) -> i32>>, // same semantics as C: <0 error, 0 unchanged, >0 replaced
    pub get_var: Option<Rc<dyn Fn(&LilInterpreter, &str, &mut LilValue) -> bool>>,
    pub write: Option<Rc<dyn Fn(&LilInterpreter, &str)>>,
    pub read: Option<Rc<dyn Fn(&LilInterpreter, &str) -> String>>,
    pub store: Option<Rc<dyn Fn(&LilInterpreter, &str, &str)>>,
    pub error: Option<Rc<dyn Fn(&LilInterpreter, usize, &str)>>,
    pub exit: Option<Rc<dyn Fn(&LilInterpreter, LilValueOpt)>>,
    pub source: Option<Rc<dyn Fn(&LilInterpreter, &str) -> String>>,
}

impl Default for LilCallbacks {
    fn default() -> Self {
        LilCallbacks {
            set_var: None,
            get_var: None,
            write: None,
            read: None,
            store: None,
            error: None,
            exit: None,
            source: None,
        }
    }
}

// ---------------- Interpreter State ----------------

const ERROR_NOERROR: i32 = 0;
const ERROR_DEFAULT: i32 = 1;
const ERROR_FIXHEAD: i32 = 2;
const MAX_CATCHER_DEPTH: usize = 16384;

pub struct LilInterpreter {
    // parsing state
    code: Vec<char>,
    root_code: String,
    clen: usize,
    head: usize,
    ignore_eol: bool,

    // functions
    functions: Vec<Rc<LilFunction>>, // order matters for reflection
    sys_funcs: usize,

    // catcher
    catcher: Option<String>,
    in_catcher: usize,

    // dollar prefix
    dollar_prefix: String,

    // environments
    root_env: Rc<RefCell<LilEnv>>, // stored also as env_stack[0]
    env: Rc<RefCell<LilEnv>>,      // current
    down_env: Option<Rc<RefCell<LilEnv>>>,

    // empty value
    empty: LilValue,

    // error state
    error: i32,
    err_head: usize,
    err_msg: String,

    callbacks: LilCallbacks,
    parse_depth: usize,

    pub user_data: Option<Box<dyn std::any::Any>>, // generic user data
}

impl LilInterpreter {
    // -------------- Construction / Destruction --------------

    pub fn new() -> Self {
        let root_env = Rc::new(RefCell::new(LilEnv::default()));
        let empty = LilValue::default();

        let mut lil = LilInterpreter {
            code: Vec::new(),
            root_code: String::new(),
            clen: 0,
            head: 0,
            ignore_eol: false,
            functions: Vec::new(),
            sys_funcs: 0,
            catcher: None,
            in_catcher: 0,
            dollar_prefix: "set ".to_string(),
            root_env: root_env.clone(),
            env: root_env,
            down_env: None,
            empty,
            error: ERROR_NOERROR,
            err_head: 0,
            err_msg: String::new(),
            callbacks: LilCallbacks::default(),
            parse_depth: 0,
            user_data: None,
        };

        lil.register_standard_commands();
        lil
    }

    // -------------- Public API equivalents --------------

    pub fn register_function(&mut self, name: &str, proc: fn(&LilInterpreter, &[LilValue]) -> LilValueOpt) {
        if let Some(f) = self.find_function(name) {
            let mut _fc = f.clone(); let f_mut = Rc::make_mut(&mut _fc);
            f_mut.implementation = LilFunctionImpl::Builtin(proc);
            return;
        }
        let func = LilFunction {
            name: name.to_string(),
            implementation: LilFunctionImpl::Builtin(proc),
        };
        self.functions.push(Rc::new(func));
    }

    pub fn set_callbacks(&mut self, callbacks: LilCallbacks) {
        self.callbacks = callbacks;
    }

    pub fn set_error(&mut self, msg: Option<&str>) {
        if self.error != ERROR_NOERROR {
            return;
        }
        self.error = ERROR_FIXHEAD;
        self.err_head = 0;
        self.err_msg = msg.unwrap_or("").to_string();
    }

    pub fn set_error_at(&mut self, pos: usize, msg: Option<&str>) {
        if self.error != ERROR_NOERROR {
            return;
        }
        self.error = ERROR_DEFAULT;
        self.err_head = pos;
        self.err_msg = msg.unwrap_or("").to_string();
    }

    pub fn take_error(&mut self) -> Option<(String, usize)> {
        if self.error == ERROR_NOERROR {
            None
        } else {
            let msg = std::mem::take(&mut self.err_msg);
            let pos = self.err_head;
            self.error = ERROR_NOERROR;
            Some((msg, pos))
        }
    }

    pub fn eval_string(&mut self, code: &str) -> LilValueOpt {
        self.parse(code, 0, false)
    }

    pub fn eval_value(&mut self, value: &LilValue) -> LilValueOpt {
        if value.is_empty() {
            return Some(LilValue::default());
        }
        self.parse(value.as_str(), value.as_str().len(), false)
    }

    pub fn get_var(&self, name: &str) -> LilValue {
        self.get_var_or(name, self.empty.clone())
    }

    pub fn get_var_or(&self, name: &str, def: LilValue) -> LilValue {
        let var = self.find_var(&self.env, name);
        let mut retval = var.as_ref().map(|v| v.value.clone()).unwrap_or(def);
        if let Some(cb) = &self.callbacks.get_var {
            // if var is None or global/root env, callback may override
            let is_global = var
                .as_ref()
                .map(|v| self.is_var_in_root(&self.env, &v.name))
                .unwrap_or(true);
            if is_global {
                let mut new_val = retval.clone();
                if cb(self, name, &mut new_val) {
                    retval = new_val;
                }
            }
        }
        retval
    }

    pub fn set_var(&mut self, name: &str, val: LilValue, local_mode: SetVarMode) -> Option<LilValue> {
        if name.is_empty() {
            return None;
        }

        let env_rc = match local_mode {
            SetVarMode::Global => self.root_env.clone(),
            _ => self.env.clone(),
        };

        // try update existing, unless LocalNew
        let mut free_val = false;
        if !matches!(local_mode, SetVarMode::LocalNew) {
            let var_opt = self.find_var(&env_rc, name);
            let mut var_opt2 = var_opt;
            if matches!(local_mode, SetVarMode::LocalOnly) {
                if let Some(v) = &var_opt2 {
                    if self.is_var_in_root(&env_rc, &v.name) && !Rc::ptr_eq(&env_rc, &self.root_env) {
                        var_opt2 = None;
                    }
                }
            }

            let mut val_clone = val.clone();
            if ((var_opt2.is_none() && Rc::ptr_eq(&env_rc, &self.root_env))
                || (var_opt2.is_some() && self.is_var_in_root(&env_rc, name)))
                && self.callbacks.set_var.is_some()
            {
                if let Some(cb) = &self.callbacks.set_var {
                    let mut newval = val_clone.clone();
                    let r = cb(self, name, &mut newval);
                    if r < 0 {
                        return None;
                    }
                    if r != 0 {
                        val_clone = newval;
                        free_val = true;
                    }
                }
            }

            if let Some(v) = var_opt2 {
                let mut env = env_rc.borrow_mut();
                if let Some(existing) = env.vars.iter_mut().rev().find(|vv| vv.name == name) {
                    existing.value = if free_val { val_clone } else { val_clone.clone() };
                    return Some(existing.value.clone());
                }
            }
        }

        // create new
        let mut env = env_rc.borrow_mut();
        let value = if free_val { val } else { val.clone() };
        env.vars.push(LilVar {
            name: name.to_string(),
            value: value.clone(),
        });
        Some(value)
    }

    // -------------- Internal helpers --------------

    fn is_var_in_root(&self, env: &Rc<RefCell<LilEnv>>, _name: &str) -> bool {
        // In C code they check var->env == lil->rootenv. Here we approximate by identity of env.
        Rc::ptr_eq(env, &self.root_env)
    }

    fn push_env(&mut self) -> Rc<RefCell<LilEnv>> {
        let new_env = Rc::new(RefCell::new(LilEnv {
            parent: Some(self.env.clone()),
            ..LilEnv::default()
        }));
        self.env = new_env.clone();
        new_env
    }

    fn pop_env(&mut self) {
        let parent = self.env.borrow().parent.clone();
        if let Some(p) = parent {
            self.env = p;
        }
    }

    fn find_local_var<'a>(&'a self, env: &Rc<RefCell<LilEnv>>, name: &str) -> Option<LilVar> {
        let e = env.borrow();
        for v in e.vars.iter().rev() {
            if v.name == name {
                return Some(v.clone());
            }
        }
        None
    }

    fn find_var<'a>(&'a self, env: &Rc<RefCell<LilEnv>>, name: &str) -> Option<LilVar> {
        if let Some(v) = self.find_local_var(env, name) {
            Some(v)
        } else if Rc::ptr_eq(env, &self.root_env) {
            None
        } else {
            self.find_var(&self.root_env, name)
        }
    }

    fn find_function(&self, name: &str) -> Option<Rc<LilFunction>> {
        for f in self.functions.iter().rev() {
            if f.name == name {
                return Some(f.clone());
            }
        }
        None
    }

    fn add_or_get_function(&mut self, name: &str) -> Rc<LilFunction> {
        if let Some(f) = self.find_function(name) {
            return f;
        }
        let func = LilFunction {
            name: name.to_string(),
            implementation: LilFunctionImpl::User {
                code: LilValue::default(),
                arg_names: LilList::default(),
            },
        };
        let rc = Rc::new(func);
        self.functions.push(rc.clone());
        rc
    }

    fn skip_spaces(&mut self) {
        while self.head < self.clen {
            let ch = self.code[self.head];
            if ch == '#' {
                // comment until eol
                while self.head < self.clen && !self.at_eol() {
                    self.head += 1;
                }
            } else if ch == '\\' && self.head + 1 < self.clen {
                let n = self.code[self.head + 1];
                if n == '\n' || n == '\r' {
                    self.head += 1;
                    while self.head < self.clen && self.at_eol() {
                        self.head += 1;
                    }
                } else if ch.is_whitespace()
                    && (self.ignore_eol
                        || !(ch == '\n' || ch == '\r'))
                {
                    self.head += 1;
                } else {
                    if ch.is_whitespace()
                        && (self.ignore_eol
                            || !(ch == '\n' || ch == '\r'))
                    {
                        self.head += 1;
                    } else {
                        break;
                    }
                }
            } else if ch.is_whitespace()
                && (self.ignore_eol || !(ch == '\n' || ch == '\r'))
            {
                self.head += 1;
            } else {
                break;
            }
        }
    }

    fn at_eol(&self) -> bool {
        if self.ignore_eol || self.head >= self.clen {
            return false;
        }
        matches!(self.code[self.head], '\n' | '\r' | ';')
    }

    fn is_special(ch: char) -> bool {
        matches!(ch, ';' | '$' | '[' | ']' | '{' | '}' | '"' | '\'')
    }

    fn get_bracket_part(&mut self) -> LilValueOpt {
        let mut cnt = 1usize;
        let mut cmd = String::new();
        self.head += 1; // skip '['
        while self.head < self.clen {
            let ch = self.code[self.head];
            if ch == '[' {
                self.head += 1;
                cnt += 1;
                cmd.push('[');
            } else if ch == ']' {
                self.head += 1;
                cnt -= 1;
                if cnt == 0 {
                    break;
                } else {
                    cmd.push(']');
                }
            } else {
                cmd.push(ch);
                self.head += 1;
            }
        }
        let val = self.parse(&cmd, cmd.len(), false);
        val
    }

    fn get_dollar_part(&mut self) -> LilValueOpt {
        self.head += 1; // skip '$'
        let name = self.next_word().unwrap_or_else(|| LilValue::default());
        let mut tmp = String::new();
        tmp.push_str(&self.dollar_prefix);
        tmp.push_str(name.as_str());
        let val = self.parse(&tmp, tmp.len(), false);
        val
    }

    fn next_word(&mut self) -> LilValueOpt {
        self.skip_spaces();
        if self.head >= self.clen {
            return Some(LilValue::default());
        }
        let ch = self.code[self.head];
        let mut val = String::new();
        if ch == '$' {
            return self.get_dollar_part();
        } else if ch == '{' {
            let mut cnt = 1usize;
            self.head += 1;
            while self.head < self.clen {
                let c = self.code[self.head];
                if c == '{' {
                    self.head += 1;
                    cnt += 1;
                    val.push('{');
                } else if c == '}' {
                    self.head += 1;
                    cnt -= 1;
                    if cnt == 0 {
                        break;
                    } else {
                        val.push('}');
                    }
                } else {
                    val.push(c);
                    self.head += 1;
                }
            }
        } else if ch == '[' {
            return self.get_bracket_part();
        } else if ch == '"' || ch == '\'' {
            let sc = ch;
            self.head += 1; // skip quote
            while self.head < self.clen {
                let c = self.code[self.head];
                if c == '[' || c == '$' {
                    let tmp = if c == '$' {
                        self.get_dollar_part()
                    } else {
                        self.get_bracket_part()
                    };
                    if let Some(t) = tmp {
                        val.push_str(t.as_str());
                    }
                    self.head -= 1; // avoid skip below
                } else if c == '\\' {
                    self.head += 1;
                    if self.head >= self.clen {
                        break;
                    }
                    let esc = self.code[self.head];
                    match esc {
                        'b' => val.push('\u{0008}'),
                        't' => val.push('\t'),
                        'n' => val.push('\n'),
                        'v' => val.push('\u{000B}'),
                        'f' => val.push('\u{000C}'),
                        'r' => val.push('\r'),
                        '0' => val.push('\0'),
                        'a' => val.push('\u{0007}'),
                        'c' => val.push('}'),
                        'o' => val.push('{'),
                        _ => val.push(esc),
                    }
                } else if c == sc {
                    self.head += 1;
                    break;
                } else {
                    val.push(c);
                }
                self.head += 1;
            }
        } else {
            while self.head < self.clen {
                let c = self.code[self.head];
                if c.is_whitespace() || Self::is_special(c) {
                    break;
                }
                val.push(c);
                self.head += 1;
            }
        }
        Some(LilValue(val))
    }

    fn substitute(&mut self) -> Option<LilList> {
        let mut words = LilList::new();
        self.skip_spaces();
        while self.head < self.clen && !self.at_eol() && self.error == ERROR_NOERROR {
            let mut w = String::new();
            loop {
                let head_before = self.head;
                let wp = self.next_word();
                if head_before == self.head {
                    // parser stuck
                    return None;
                }
                if let Some(wp_val) = wp {
                    w.push_str(wp_val.as_str());
                }
                if self.head >= self.clen
                    || self.at_eol()
                    || self.code[self.head].is_whitespace()
                    || self.error != ERROR_NOERROR
                {
                    break;
                }
            }
            self.skip_spaces();
            words.push(LilValue(w));
        }
        Some(words)
    }

    fn subst_to_list(&mut self, code: &LilValue) -> Option<LilList> {
        let save_code = std::mem::take(&mut self.code);
        let save_clen = self.clen;
        let save_head = self.head;
        let save_ignore_eol = self.ignore_eol;

        self.code = code.as_str().chars().collect();
        self.clen = self.code.len();
        self.head = 0;
        self.ignore_eol = true;
        let words = self.substitute();

        self.code = save_code;
        self.clen = save_clen;
        self.head = save_head;
        self.ignore_eol = save_ignore_eol;
        words
    }

    fn needs_escape(s: &str) -> bool {
        if s.is_empty() {
            return true;
        }
        for ch in s.chars() {
            if ch.is_ascii_punctuation() || ch.is_ascii_whitespace() {
                return true;
            }
        }
        false
    }

    fn list_to_value(&self, list: &LilList, do_escape: bool) -> LilValue {
        let mut out = String::new();
        for (i, v) in list.0.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            let s = v.as_str();
            let esc = if do_escape {
                Self::needs_escape(s)
            } else {
                false
            };
            if esc {
                out.push('{');
            }
            out.push_str(s);
            if esc {
                out.push('}');
            }
        }
        LilValue(out)
    }

    fn subst_to_value(&mut self, code: &LilValue) -> LilValue {
        let words = self.subst_to_list(code);
        match words {
            None => code.clone(),
            Some(list) => self.list_to_value(&list, false),
        }
    }

    fn parse(&mut self, code: &str, codelen: usize, func_level: bool) -> LilValueOpt {
        let save_code = std::mem::take(&mut self.code);
        let save_clen = self.clen;
        let save_head = self.head;

        if self.parse_depth == 0 {
            self.root_code = code.to_string();
        }

        self.code = code.chars().collect();
        self.clen = if codelen > 0 { codelen } else { self.code.len() };
        self.head = 0;

        self.skip_spaces();
        self.parse_depth += 1;
        if self.parse_depth == 1 {
            self.error = ERROR_NOERROR;
        }
        if func_level {
            self.env.borrow_mut().break_run = false;
        }

        let mut val: LilValueOpt = None;

        while self.head < self.clen && self.error == ERROR_NOERROR {
            let words_opt = self.substitute();
            if self.error != ERROR_NOERROR {
                break;
            }
            let words = match words_opt {
                None => break,
                Some(w) => w,
            };
            if words.len() > 0 {
                let cmd_name = words.get(0).unwrap().as_str().to_string();
                let mut cmd = self.find_function(&cmd_name);

                if cmd.is_none() && !cmd_name.is_empty() {
                    if let Some(catcher_code) = self.catcher.clone() {
                        if self.in_catcher < MAX_CATCHER_DEPTH {
                            self.in_catcher += 1;
                            let args = self.list_to_value(&words, true);
                            let saved_env = self.env.clone();
                            let new_env = self.push_env();
                            new_env.borrow_mut().catcher_for = Some(words.get(0).unwrap().clone());
                            self.set_var("args", args, SetVarMode::LocalNew);
                            val = self.parse(&catcher_code, 0, true);
                            self.env = saved_env;
                            self.in_catcher -= 1;
                        } else {
                            let msg = format!(
                                "catcher limit reached while trying to call unknown function {}",
                                cmd_name
                            );
                            self.set_error_at(self.head, Some(&msg));
                            break;
                        }
                    } else {
                        let msg = format!("unknown function {}", cmd_name);
                        self.set_error_at(self.head, Some(&msg));
                        break;
                    }
                }

                if let Some(cmd_rc) = cmd.take() {
                    match &cmd_rc.implementation {
                        LilFunctionImpl::Builtin(proc) => {
                            let shead = self.head;
                            let args: Vec<LilValue> = words.0.into_iter().skip(1).collect();
                            val = proc(self, &args);
                            if self.error == ERROR_FIXHEAD {
                                self.error = ERROR_DEFAULT;
                                self.err_head = shead;
                            }
                        }
                        LilFunctionImpl::User { code, arg_names } => {
                            let saved_env = self.env.clone();
                            let new_env = self.push_env();
                            new_env.borrow_mut().func = Some(cmd_rc.clone());

                            if arg_names.len() == 1
                                && arg_names
                                    .get(0)
                                    .map(|v| v.as_str() == "args")
                                    .unwrap_or(false)
                            {
                                let args_value = self.list_to_value(&words, true);
                                self.set_var("args", args_value, SetVarMode::LocalNew);
                            } else {
                                for (i, arg_name) in arg_names.0.iter().enumerate() {
                                    let v = if i + 1 < words.len() {
                                        words.get(i + 1).unwrap().clone()
                                    } else {
                                        self.empty.clone()
                                    };
                                    self.set_var(arg_name.as_str(), v, SetVarMode::LocalNew);
                                }
                            }

                            val = self.eval_value(code);
                            self.env = saved_env;
                        }
                    }
                }
            }

            if self.env.borrow().break_run {
                break;
            }

            self.skip_spaces();
            while self.head < self.clen && self.at_eol() {
                self.head += 1;
            }
            self.skip_spaces();
        }

        if self.error != ERROR_NOERROR && self.parse_depth == 1 {
            if let Some(cb) = &self.callbacks.error {
                cb(self, self.err_head, &self.err_msg);
            }
        }

        // restore
        self.code = save_code;
        self.clen = save_clen;
        self.head = save_head;

        if func_level {
            let mut env = self.env.borrow_mut();
            if env.retval_set {
                val = env.retval.take();
                env.retval_set = false;
                env.break_run = false;
            }
        }

        self.parse_depth -= 1;
        if val.is_none() {
            Some(LilValue::default())
        } else {
            val
        }
    }

    // -------------- Expression Evaluator --------------

    fn eval_expr_value(&mut self, code: &LilValue) -> LilValueOpt {
        let substituted = self.subst_to_value(code);
        if self.error != ERROR_NOERROR {
            return None;
        }
        let s = substituted.as_str();
        if s.is_empty() {
            return Some(LilValue::new("0"));
        }
        let mut ee = ExprEval::new(s);
        ee.expr();
        if ee.error != ExprError::NoError {
            match ee.error {
                ExprError::DivZero => self.set_error(Some("division by zero in expression")),
                ExprError::InvalidType => self.set_error(Some("mixing invalid types in expression")),
                ExprError::SyntaxError => self.set_error(Some("expression syntax error")),
                _ => {}
            }
            return None;
        }
        match ee.ty {
            ExprType::Int => Some(LilValue::new(format!("{}", ee.ival))),
            ExprType::Float => Some(LilValue::new(format!("{}", ee.dval))),
        }
    }

    // -------------- Standard command registration --------------

    fn register_standard_commands(&mut self) {
        self.register_function("reflect", Self::fnc_reflect);
        self.register_function("func", Self::fnc_func);
        self.register_function("rename", Self::fnc_rename);
        self.register_function("unusedname", Self::fnc_unusedname);
        self.register_function("quote", Self::fnc_quote);
        self.register_function("set", Self::fnc_set);
        self.register_function("local", Self::fnc_local);
        self.register_function("write", Self::fnc_write);
        self.register_function("print", Self::fnc_print);
        self.register_function("eval", Self::fnc_eval);
        self.register_function("topeval", Self::fnc_topeval);
        self.register_function("upeval", Self::fnc_upeval);
        self.register_function("downeval", Self::fnc_downeval);
        self.register_function("enveval", Self::fnc_enveval);
        self.register_function("jaileval", Self::fnc_jaileval);
        self.register_function("count", Self::fnc_count);
        self.register_function("index", Self::fnc_index);
        self.register_function("indexof", Self::fnc_indexof);
        self.register_function("filter", Self::fnc_filter);
        self.register_function("list", Self::fnc_list);
        self.register_function("append", Self::fnc_append);
        self.register_function("slice", Self::fnc_slice);
        self.register_function("subst", Self::fnc_subst);
        self.register_function("concat", Self::fnc_concat);
        self.register_function("foreach", Self::fnc_foreach);
        self.register_function("return", Self::fnc_return);
        self.register_function("result", Self::fnc_result);
        self.register_function("expr", Self::fnc_expr);
        self.register_function("inc", Self::fnc_inc);
        self.register_function("dec", Self::fnc_dec);
        self.register_function("read", Self::fnc_read);
        self.register_function("store", Self::fnc_store);
        self.register_function("if", Self::fnc_if);
        self.register_function("while", Self::fnc_while);
        self.register_function("for", Self::fnc_for);
        self.register_function("char", Self::fnc_char);
        self.register_function("charat", Self::fnc_charat);
        self.register_function("codeat", Self::fnc_codeat);
        self.register_function("substr", Self::fnc_substr);
        self.register_function("strpos", Self::fnc_strpos);
        self.register_function("length", Self::fnc_length);
        self.register_function("trim", Self::fnc_trim);
        self.register_function("ltrim", Self::fnc_ltrim);
        self.register_function("rtrim", Self::fnc_rtrim);
        self.register_function("strcmp", Self::fnc_strcmp);
        self.register_function("streq", Self::fnc_streq);
        self.register_function("repstr", Self::fnc_repstr);
        self.register_function("split", Self::fnc_split);
        self.register_function("try", Self::fnc_try);
        self.register_function("error", Self::fnc_error);
        self.register_function("exit", Self::fnc_exit);
        self.register_function("source", Self::fnc_source);
        self.register_function("lmap", Self::fnc_lmap);
        self.register_function("rand", Self::fnc_rand);
        self.register_function("catcher", Self::fnc_catcher);

        self.sys_funcs = self.functions.len();
    }

    // -------------- Utility conversions --------------

    fn to_bool(val: &LilValue) -> bool {
        let s = val.as_str();
        if s.is_empty() {
            return false;
        }
        let mut dots = 0;
        for ch in s.chars() {
            if ch != '0' && ch != '.' {
                return true;
            }
            if ch == '.' {
                if dots > 0 {
                    return true;
                }
                dots += 1;
            }
        }
        false
    }

    fn to_int(val: &LilValue) -> LilInt {
        val.as_str().parse::<LilInt>().unwrap_or(0)
    }

    fn to_float(val: &LilValue) -> f64 {
        val.as_str().parse::<f64>().unwrap_or(0.0)
    }

    fn unused_name(&self, part: &str) -> Option<LilValue> {
        for i in 0..=u32::MAX {
            let name = format!("!!un!{}!{:09}!nu!!", part, i);
            if self.find_function(&name).is_some() {
                continue;
            }
            if self.find_var(&self.env, &name).is_some() {
                continue;
            }
            return Some(LilValue::new(name));
        }
        None
    }

    // -------------- Builtin commands (one-to-one with C) --------------

    fn fnc_reflect(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let t = argv[0].as_str();
        match t {
            "version" => Some(LilValue::new("LIL-Rust")),
            "args" => {
                if argv.len() < 2 {
                    return None;
                }
                let func_name = argv[1].as_str();
                let func = interp.find_function(func_name)?;
                if let LilFunctionImpl::User { arg_names, .. } = &func.implementation {
                    Some(interp.list_to_value(arg_names, true))
                } else {
                    None
                }
            }
            "body" => {
                if argv.len() < 2 {
                    return None;
                }
                let func_name = argv[1].as_str();
                let func = interp.find_function(func_name)?;
                if let LilFunctionImpl::User { code, .. } = &func.implementation {
                    Some(code.clone())
                } else {
                    None
                }
            }
            "func-count" => Some(LilValue::new(format!("{}", interp.functions.len()))),
            "funcs" => {
                let mut list = LilList::new();
                for f in &interp.functions {
                    list.push(LilValue::new(f.name.clone()));
                }
                Some(interp.list_to_value(&list, true))
            }
            "vars" => {
                let mut vars = LilList::new();
                let mut env_opt = Some(interp.env.clone());
                while let Some(env_rc) = env_opt {
                    let env = env_rc.borrow();
                    for v in &env.vars {
                        vars.push(LilValue::new(v.name.clone()));
                    }
                    env_opt = env.parent.clone();
                }
                Some(interp.list_to_value(&vars, true))
            }
            "globals" => {
                let mut vars = LilList::new();
                let env = interp.root_env.borrow();
                for v in &env.vars {
                    vars.push(LilValue::new(v.name.clone()));
                }
                Some(interp.list_to_value(&vars, true))
            }
            "has-func" => {
                if argv.len() < 2 {
                    return None;
                }
                let target = argv[1].as_str();
                for f in &interp.functions {
                    if f.name == target {
                        return Some(LilValue::new("1"));
                    }
                }
                None
            }
            "has-var" => {
                if argv.len() < 2 {
                    return None;
                }
                let target = argv[1].as_str();
                let mut env_opt = Some(interp.env.clone());
                while let Some(env_rc) = env_opt {
                    let env = env_rc.borrow();
                    for v in &env.vars {
                        if v.name == target {
                            return Some(LilValue::new("1"));
                        }
                    }
                    env_opt = env.parent.clone();
                }
                None
            }
            "has-global" => {
                if argv.len() < 2 {
                    return None;
                }
                let target = argv[1].as_str();
                let env = interp.root_env.borrow();
                for v in &env.vars {
                    if v.name == target {
                        return Some(LilValue::new("1"));
                    }
                }
                None
            }
            "error" => {
                if interp.err_msg.is_empty() {
                    None
                } else {
                    Some(LilValue::new(interp.err_msg.clone()))
                }
            }
            "dollar-prefix" => {
                if argv.len() == 1 {
                    Some(LilValue::new(interp.dollar_prefix.clone()))
                } else {
                    let old = LilValue::new(interp.dollar_prefix.clone());
                    let mut interp_mut = interp as *const _ as *mut LilInterpreter;
                    unsafe {
                        (*interp_mut).dollar_prefix = argv[1].as_str().to_string();
                    }
                    Some(old)
                }
            }
            "this" => {
                let mut env_rc = interp.env.clone();
                loop {
                    let env = env_rc.borrow();
                    if Rc::ptr_eq(&env_rc, &interp.root_env)
                        || env.catcher_for.is_some()
                        || env.func.is_some()
                    {
                        if env.catcher_for.is_some() {
                            return interp
                                .catcher
                                .as_ref()
                                .map(|c| LilValue::new(c.clone()));
                        }
                        if Rc::ptr_eq(&env_rc, &interp.root_env) {
                            return Some(LilValue::new(interp.root_code.clone()));
                        }
                        if let Some(f) = &env.func {
                            if let LilFunctionImpl::User { code, .. } = &f.implementation {
                                return Some(code.clone());
                            }
                        }
                        return None;
                    }
                    let _parent = env.parent.clone();
                    drop(env);
                    if let Some(parent) = _parent {
                        env_rc = parent;
                    } else {
                        break;
                    }
                }
                None
            }
            "name" => {
                let mut env_rc = interp.env.clone();
                loop {
                    let env = env_rc.borrow();
                    if Rc::ptr_eq(&env_rc, &interp.root_env)
                        || env.catcher_for.is_some()
                        || env.func.is_some()
                    {
                        if let Some(cf) = &env.catcher_for {
                            return Some(cf.clone());
                        }
                        if Rc::ptr_eq(&env_rc, &interp.root_env) {
                            return None;
                        }
                        if let Some(f) = &env.func {
                            return Some(LilValue::new(f.name.clone()));
                        }
                        return None;
                    }
                    let _parent = env.parent.clone();
                    drop(env);
                    if let Some(parent) = _parent {
                        env_rc = parent;
                    } else {
                        break;
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn fnc_func(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            if argv.is_empty() {
                return None;
            }
            let name_val: LilValue;
            let arg_names: LilList;
            let code: LilValue;
            if argv.len() == 3 {
                name_val = argv[0].clone();
                let func = (*interp_mut).add_or_get_function(argv[0].as_str());
                arg_names = (*interp_mut).subst_to_list(&argv[1]).unwrap_or_default();
                code = argv[2].clone();
                let mut _fc = func.clone(); let f = Rc::make_mut(&mut _fc);
                f.implementation = LilFunctionImpl::User { code: code.clone(), arg_names: arg_names.clone() };
            } else {
                let unused = (*interp_mut).unused_name("anonymous-function");
                if unused.is_none() {
                    return None;
                }
                name_val = unused.unwrap();
                let func = (*interp_mut).add_or_get_function(name_val.as_str());
                if argv.len() < 2 {
                    let tmp = LilValue::new("args");
                    arg_names = (*interp_mut).subst_to_list(&tmp).unwrap_or_default();
                    code = argv[0].clone();
                } else {
                    arg_names = (*interp_mut).subst_to_list(&argv[0]).unwrap_or_default();
                    code = argv[1].clone();
                }
                let mut _fc = func.clone(); let f = Rc::make_mut(&mut _fc);
                f.implementation = LilFunctionImpl::User { code: code.clone(), arg_names: arg_names.clone() };
            }
            Some(name_val)
        }
    }

    fn fnc_rename(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let oldname = argv[0].as_str();
        let newname = argv[1].as_str();
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let func_opt = (*interp_mut).find_function(oldname);
            let func = match func_opt {
                None => {
                    let msg = format!("unknown function '{}'", oldname);
                    (*interp_mut).set_error_at((*interp_mut).head, Some(&msg));
                    return None;
                }
                Some(f) => f,
            };
            let old = LilValue::new(func.name.clone());
            let mut _fc = func.clone(); let f = Rc::make_mut(&mut _fc);
            f.name = newname.to_string();
            Some(old)
        }
    }

    fn fnc_unusedname(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let part = if argv.is_empty() {
            "unusedname"
        } else {
            argv[0].as_str()
        };
        interp.unused_name(part)
    }

    fn fnc_quote(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut out = String::new();
        for (i, v) in argv.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            out.push_str(v.as_str());
        }
        Some(LilValue::new(out))
    }

    fn fnc_set(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let mut i = 0usize;
            let mut access = SetVarMode::Local;
            let mut last: Option<LilValue> = None;
            if argv[0].as_str() == "global" {
                i = 1;
                access = SetVarMode::Global;
            }
            while i < argv.len() {
                if argv.len() == i + 1 {
                    return Some((*interp_mut).get_var(argv[i].as_str()));
                }
                let name = argv[i].as_str();
                let v = argv[i + 1].clone();
                last = (*interp_mut).set_var(name, v, access);
                i += 2;
            }
            last
        }
    }

    fn fnc_local(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            for v in argv {
                let varname = v.as_str();
                if (*interp_mut).find_local_var(&(*interp_mut).env, varname).is_none() {
                    (*interp_mut).set_var(varname, (*interp_mut).empty.clone(), SetVarMode::LocalNew);
                }
            }
        }
        None
    }

    fn fnc_write(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut msg = String::new();
        for (i, v) in argv.iter().enumerate() {
            if i > 0 {
                msg.push(' ');
            }
            msg.push_str(v.as_str());
        }
        if let Some(cb) = &interp.callbacks.write {
            cb(interp, &msg);
        } else {
            print!("{}", msg);
        }
        None
    }

    fn fnc_print(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        Self::fnc_write(interp, argv);
        if let Some(cb) = &interp.callbacks.write {
            cb(interp, "\n");
        } else {
            println!();
        }
        None
    }

    fn fnc_eval(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            if argv.len() == 1 {
                (*interp_mut).eval_value(&argv[0])
            } else if !argv.is_empty() {
                let mut s = String::new();
                for (i, v) in argv.iter().enumerate() {
                    if i > 0 {
                        s.push(' ');
                    }
                    s.push_str(v.as_str());
                }
                (*interp_mut).parse(&s, 0, false)
            } else {
                None
            }
        }
    }

    fn fnc_topeval(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let this_env = (*interp_mut).env.clone();
            let this_down = (*interp_mut).down_env.clone();
            (*interp_mut).env = (*interp_mut).root_env.clone();
            (*interp_mut).down_env = Some(this_env.clone());
            let r = Self::fnc_eval(&*interp_mut, argv);
            (*interp_mut).down_env = this_down;
            (*interp_mut).env = this_env;
            r
        }
    }

    fn fnc_upeval(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let this_env = (*interp_mut).env.clone();
            let this_down = (*interp_mut).down_env.clone();
            if Rc::ptr_eq(&this_env, &(*interp_mut).root_env) {
                return Self::fnc_eval(&*interp_mut, argv);
            }
            let parent = this_env.borrow().parent.clone().unwrap();
            (*interp_mut).env = parent.clone();
            (*interp_mut).down_env = Some(this_env.clone());
            let r = Self::fnc_eval(&*interp_mut, argv);
            (*interp_mut).env = this_env;
            (*interp_mut).down_env = this_down;
            r
        }
    }

    fn fnc_downeval(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let up_env = (*interp_mut).env.clone();
            let down_env = (*interp_mut).down_env.clone();
            if down_env.is_none() {
                return Self::fnc_eval(&*interp_mut, argv);
            }
            let down = down_env.unwrap();
            (*interp_mut).down_env = None;
            (*interp_mut).env = down.clone();
            let r = Self::fnc_eval(&*interp_mut, argv);
            (*interp_mut).down_env = Some(down);
            (*interp_mut).env = up_env;
            r
        }
    }

    fn fnc_enveval(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            if argv.is_empty() {
                return None;
            }
            let mut invars: Option<LilList> = None;
            let mut outvars: Option<LilList> = None;
            let mut varvalues: Vec<LilValue> = Vec::new();
            let code_index: usize;

            if argv.len() == 1 {
                code_index = 0;
            } else {
                let iv = (*interp_mut).subst_to_list(&argv[0]);
                if let Some(l) = iv.clone() {
                    for v in &l.0 {
                        varvalues.push((*interp_mut).get_var(v.as_str()));
                    }
                }
                invars = iv;
                if argv.len() > 2 {
                    code_index = 2;
                    outvars = (*interp_mut).subst_to_list(&argv[1]);
                } else {
                    code_index = 1;
                }
            }

            let saved_env = (*interp_mut).env.clone();
            let new_env = (*interp_mut).push_env();
            if let Some(iv) = &invars {
                for (i, vname) in iv.0.iter().enumerate() {
                    let val = varvalues[i].clone();
                    (*interp_mut).set_var(vname.as_str(), val, SetVarMode::LocalNew);
                }
            }
            let r = (*interp_mut).eval_value(&argv[code_index]);

            if invars.is_some() || outvars.is_some() {
                varvalues.clear();
                if let Some(ov) = &outvars {
                    for v in &ov.0 {
                        varvalues.push((*interp_mut).get_var(v.as_str()));
                    }
                } else if let Some(iv) = &invars {
                    for v in &iv.0 {
                        varvalues.push((*interp_mut).get_var(v.as_str()));
                    }
                }
            }

            (*interp_mut).env = saved_env;

            if let Some(iv) = invars {
                if let Some(ov) = outvars {
                    for (i, v) in ov.0.iter().enumerate() {
                        (*interp_mut).set_var(v.as_str(), varvalues[i].clone(), SetVarMode::Local);
                    }
                } else {
                    for (i, v) in iv.0.iter().enumerate() {
                        (*interp_mut).set_var(v.as_str(), varvalues[i].clone(), SetVarMode::Local);
                    }
                }
            }
            r
        }
    }

    fn fnc_jaileval(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut base = 0usize;
        if argv[0].as_str() == "clean" {
            base = 1;
            if argv.len() == 1 {
                return None;
            }
        }
        let mut sub = LilInterpreter::new();
        if base != 1 {
            for f in &interp.functions[interp.sys_funcs..] {
                if let LilFunctionImpl::Builtin(proc) = f.implementation {
                    sub.register_function(&f.name, proc);
                }
            }
        }
        sub.eval_value(&argv[base])
    }

    fn fnc_count(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return Some(LilValue::new("0"));
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let list = (*interp_mut).subst_to_list(&argv[0]).unwrap_or_default();
            Some(LilValue::new(format!("{}", list.len())))
        }
    }

    fn fnc_index(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let list = (*interp_mut).subst_to_list(&argv[0]).unwrap_or_default();
            let index = Self::to_int(&argv[1]) as usize;
            if index >= list.len() {
                None
            } else {
                list.get(index).cloned()
            }
        }
    }

    fn fnc_indexof(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let list = (*interp_mut).subst_to_list(&argv[0]).unwrap_or_default();
            let target = argv[1].as_str();
            for (i, v) in list.0.iter().enumerate() {
                if v.as_str() == target {
                    return Some(LilValue::new(format!("{}", i)));
                }
            }
            None
        }
    }

    fn fnc_append(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let mut base = 1usize;
            let mut access = SetVarMode::Local;
            let mut varname = argv[0].as_str();
            if varname == "global" {
                if argv.len() < 3 {
                    return None;
                }
                varname = argv[1].as_str();
                base = 2;
                access = SetVarMode::Global;
            }
            let current = (*interp_mut).get_var(varname);
            let mut list = (*interp_mut).subst_to_list(&current).unwrap_or_default();
            for v in &argv[base..] {
                list.push(v.clone());
            }
            let r = (*interp_mut).list_to_value(&list, true);
            (*interp_mut).set_var(varname, r.clone(), access);
            Some(r)
        }
    }

    fn fnc_slice(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        if argv.len() < 2 {
            return Some(argv[0].clone());
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let mut from = Self::to_int(&argv[1]);
            if from < 0 {
                from = 0;
            }
            let list = (*interp_mut).subst_to_list(&argv[0]).unwrap_or_default();
            let mut to = if argv.len() > 2 {
                Self::to_int(&argv[2])
            } else {
                list.len() as LilInt
            };
            if to > list.len() as LilInt {
                to = list.len() as LilInt;
            }
            if to < from {
                to = from;
            }
            let mut slice = LilList::new();
            for i in from as usize..to as usize {
                if let Some(v) = list.get(i) {
                    slice.push(v.clone());
                }
            }
            Some((*interp_mut).list_to_value(&slice, true))
        }
    }

    fn fnc_filter(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let mut varname = "x";
            let mut base = 0usize;
            if argv.len() > 2 {
                varname = argv[0].as_str();
                base = 1;
            }
            let list = (*interp_mut).subst_to_list(&argv[base]).unwrap_or_default();
            let mut filtered = LilList::new();
            for v in &list.0 {
                (*interp_mut).set_var(varname, v.clone(), SetVarMode::LocalOnly);
                let r = (*interp_mut).eval_expr_value(&argv[base + 1]);
                if let Some(rv) = r {
                    if Self::to_bool(&rv) {
                        filtered.push(v.clone());
                    }
                }
                if (*interp_mut).env.borrow().break_run || (*interp_mut).error != ERROR_NOERROR {
                    break;
                }
            }
            Some((*interp_mut).list_to_value(&filtered, true))
        }
    }

    fn fnc_list(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut list = LilList::new();
        for v in argv {
            list.push(v.clone());
        }
        Some(interp.list_to_value(&list, true))
    }

    fn fnc_subst(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe { Some((*interp_mut).subst_to_value(&argv[0])) }
    }

    fn fnc_concat(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let mut r = String::new();
            for v in argv {
                let list = (*interp_mut).subst_to_list(v).unwrap_or_default();
                let tmp = (*interp_mut).list_to_value(&list, true);
                r.push_str(tmp.as_str());
            }
            Some(LilValue::new(r))
        }
    }

    fn fnc_foreach(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let mut list_idx = 0usize;
            let mut code_idx = 1usize;
            let mut varname = "i";
            if argv.len() >= 3 {
                varname = argv[0].as_str();
                list_idx = 1;
                code_idx = 2;
            }
            let list = (*interp_mut).subst_to_list(&argv[list_idx]).unwrap_or_default();
            let mut rlist = LilList::new();
            for v in &list.0 {
                (*interp_mut).set_var(varname, v.clone(), SetVarMode::LocalOnly);
                let rv = (*interp_mut).eval_value(&argv[code_idx]).unwrap_or_default();
                if !rv.is_empty() {
                    rlist.push(rv);
                }
                if (*interp_mut).env.borrow().break_run || (*interp_mut).error != ERROR_NOERROR {
                    break;
                }
            }
            Some((*interp_mut).list_to_value(&rlist, true))
        }
    }

    fn fnc_return(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut env = interp.env.borrow_mut();
        env.break_run = true;
        env.retval = if argv.is_empty() {
            None
        } else {
            Some(argv[0].clone())
        };
        env.retval_set = true;
        if argv.is_empty() {
            None
        } else {
            Some(argv[0].clone())
        }
    }

    fn fnc_result(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut env = interp.env.borrow_mut();
        if !argv.is_empty() {
            env.retval = Some(argv[0].clone());
            env.retval_set = true;
        }
        env.retval.clone()
    }

    fn fnc_expr(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            if argv.len() == 1 {
                (*interp_mut).eval_expr_value(&argv[0])
            } else {
                let mut s = String::new();
                for (i, v) in argv.iter().enumerate() {
                    if i > 0 {
                        s.push(' ');
                    }
                    s.push_str(v.as_str());
                }
                let v = LilValue::new(s);
                (*interp_mut).eval_expr_value(&v)
            }
        }
    }

    fn real_inc(interp: &mut LilInterpreter, varname: &str, v: f64) -> LilValueOpt {
        let pv = interp.get_var(varname);
        let dv = Self::to_float(&pv) + v;
        let val = if (dv.fract()).abs() > 0.0 {
            LilValue::new(format!("{}", dv))
        } else {
            LilValue::new(format!("{}", Self::to_int(&pv) + v as LilInt))
        };
        interp.set_var(varname, val.clone(), SetVarMode::Local);
        Some(val)
    }

    fn fnc_inc(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let amount = if argv.len() > 1 {
                Self::to_float(&argv[1])
            } else {
                1.0
            };
            Self::real_inc(&mut *interp_mut, argv[0].as_str(), amount)
        }
    }

    fn fnc_dec(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let amount = if argv.len() > 1 {
                Self::to_float(&argv[1])
            } else {
                1.0
            };
            Self::real_inc(&mut *interp_mut, argv[0].as_str(), -amount)
        }
    }

    fn fnc_read(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let path = argv[0].as_str();
        let buffer = if let Some(cb) = &interp.callbacks.read {
            cb(interp, path)
        } else {
            let mut f = match File::open(Path::new(path)) {
                Ok(f) => f,
                Err(_) => return None,
            };
            let mut buf = String::new();
            if f.read_to_string(&mut buf).is_err() {
                return None;
            }
            buf
        };
        Some(LilValue::new(buffer))
    }

    fn fnc_store(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let path = argv[0].as_str();
        let data = argv[1].as_str();
        if let Some(cb) = &interp.callbacks.store {
            cb(interp, path, data);
        } else {
            let mut f = match File::create(Path::new(path)) {
                Ok(f) => f,
                Err(_) => return None,
            };
            if f.write_all(data.as_bytes()).is_err() {
                return None;
            }
        }
        Some(argv[1].clone())
    }

    fn fnc_if(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut base = 0usize;
        let mut not = false;
        if argv[0].as_str() == "not" {
            base = 1;
            not = true;
        }
        if argv.len() < base + 2 {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let cond = (*interp_mut).eval_expr_value(&argv[base]);
            if cond.is_none() || (*interp_mut).error != ERROR_NOERROR {
                return None;
            }
            let mut v = Self::to_bool(&cond.unwrap());
            if not {
                v = !v;
            }
            if v {
                (*interp_mut).eval_value(&argv[base + 1])
            } else if argv.len() > base + 2 {
                (*interp_mut).eval_value(&argv[base + 2])
            } else {
                None
            }
        }
    }

    fn fnc_while(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let mut base = 0usize;
        let mut not = false;
        if argv[0].as_str() == "not" {
            base = 1;
            not = true;
        }
        if argv.len() < base + 2 {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let mut r: LilValueOpt = None;
            loop {
                let cond = (*interp_mut).eval_expr_value(&argv[base]);
                if cond.is_none() || (*interp_mut).error != ERROR_NOERROR {
                    return None;
                }
                let mut v = Self::to_bool(&cond.unwrap());
                if not {
                    v = !v;
                }
                if !v {
                    break;
                }
                r = (*interp_mut).eval_value(&argv[base + 1]);
                if (*interp_mut).env.borrow().break_run || (*interp_mut).error != ERROR_NOERROR {
                    break;
                }
            }
            r
        }
    }

    fn fnc_for(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 4 {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            (*interp_mut).eval_value(&argv[0]);
            let mut r: LilValueOpt = None;
            loop {
                let cond = (*interp_mut).eval_expr_value(&argv[1]);
                if cond.is_none() || (*interp_mut).error != ERROR_NOERROR {
                    return None;
                }
                if !Self::to_bool(&cond.unwrap()) {
                    break;
                }
                r = (*interp_mut).eval_value(&argv[3]);
                (*interp_mut).eval_value(&argv[2]);
                if (*interp_mut).env.borrow().break_run || (*interp_mut).error != ERROR_NOERROR {
                    break;
                }
            }
            r
        }
    }

    fn fnc_char(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let code = Self::to_int(&argv[0]) as u8;
        let s = String::from(code as char);
        Some(LilValue::new(s))
    }

    fn fnc_charat(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let s = argv[0].as_str();
        let idx = Self::to_int(&argv[1]) as usize;
        if idx >= s.len() {
            return None;
        }
        let ch = s.chars().nth(idx).unwrap_or('\0');
        Some(LilValue::new(ch.to_string()))
    }

    fn fnc_codeat(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let s = argv[0].as_str();
        let idx = Self::to_int(&argv[1]) as usize;
        if idx >= s.len() {
            return None;
        }
        let ch = s.chars().nth(idx).unwrap_or('\0') as u32;
        Some(LilValue::new(format!("{}", ch)))
    }

    fn fnc_substr(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let s = argv[0].as_str();
        if s.is_empty() {
            return None;
        }
        let len = s.chars().count();
        let start = argv[1].as_str().parse::<usize>().unwrap_or(0);
        let end = if argv.len() > 2 {
            argv[2].as_str().parse::<usize>().unwrap_or(len)
        } else {
            len
        };
        if start >= end || start >= len {
            return None;
        }
        let sub: String = s.chars().skip(start).take(end - start).collect();
        Some(LilValue::new(sub))
    }

    fn fnc_strpos(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return Some(LilValue::new("-1"));
        }
        let hay = argv[0].as_str();
        let needle = argv[1].as_str();
        let mut min = 0usize;
        if argv.len() > 2 {
            min = argv[2].as_str().parse::<usize>().unwrap_or(0);
            if min >= hay.len() {
                return Some(LilValue::new("-1"));
            }
        }
        if let Some(pos) = hay[min..].find(needle) {
            Some(LilValue::new(format!("{}", min + pos)))
        } else {
            Some(LilValue::new("-1"))
        }
    }

    fn fnc_length(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut total = 0usize;
        for (i, v) in argv.iter().enumerate() {
            if i > 0 {
                total += 1;
            }
            total += v.as_str().len();
        }
        Some(LilValue::new(format!("{}", total)))
    }

    fn real_trim(s: &str, chars: &str, left: bool, right: bool) -> LilValueOpt {
        let mut base = 0usize;
        let mut end = s.len();
        let chars: Vec<char> = chars.chars().collect();
        if left {
            for (i, ch) in s.chars().enumerate() {
                if chars.contains(&ch) {
                    base = i + ch.len_utf8();
                } else {
                    base = i * ch.len_utf8();
                    break;
                }
            }
        }
        if right {
            let mut idx = s.len();
            for ch in s.chars().rev() {
                if chars.contains(&ch) {
                    idx -= ch.len_utf8();
                } else {
                    break;
                }
            }
            end = idx;
        }
        if base >= end {
            Some(LilValue::new(""))
        } else {
            Some(LilValue::new(&s[base..end]))
        }
    }

    fn fnc_trim(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let chars = if argv.len() < 2 {
            " \u{000C}\n\r\t\u{000B}"
        } else {
            argv[1].as_str()
        };
        Self::real_trim(argv[0].as_str(), chars, true, true)
    }

    fn fnc_ltrim(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let chars = if argv.len() < 2 {
            " \u{000C}\n\r\t\u{000B}"
        } else {
            argv[1].as_str()
        };
        Self::real_trim(argv[0].as_str(), chars, true, false)
    }

    fn fnc_rtrim(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let chars = if argv.len() < 2 {
            " \u{000C}\n\r\t\u{000B}"
        } else {
            argv[1].as_str()
        };
        Self::real_trim(argv[0].as_str(), chars, false, true)
    }

    fn fnc_strcmp(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let a = argv[0].as_str();
        let b = argv[1].as_str();
        let cmp = a.cmp(b) as i32; // Ordering to -1/0/1 is more complex; for now 0, 1 or -1 isn't preserved exactly
        Some(LilValue::new(format!("{}", cmp)))
    }

    fn fnc_streq(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let eq = if argv[0].as_str() == argv[1].as_str() {
            1
        } else {
            0
        };
        Some(LilValue::new(format!("{}", eq)))
    }

    fn fnc_repstr(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        if argv.len() < 3 {
            return Some(argv[0].clone());
        }
        let from = argv[1].as_str();
        let to = argv[2].as_str();
        if from.is_empty() {
            return None;
        }
        let mut s = argv[0].as_str().to_string();
        while let Some(pos) = s.find(from) {
            s.replace_range(pos..pos + from.len(), to);
        }
        Some(LilValue::new(s))
    }

    fn fnc_split(_interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let sep = if argv.len() > 1 {
            argv[1].as_str()
        } else {
            " "
        };
        if sep.is_empty() {
            return Some(argv[0].clone());
        }
        let mut list = LilList::new();
        let mut cur = String::new();
        for ch in argv[0].as_str().chars() {
            if sep.contains(ch) {
                list.push(LilValue::new(cur.clone()));
                cur.clear();
            } else {
                cur.push(ch);
            }
        }
        list.push(LilValue::new(cur));
        Some(LilValue::new(
            list.0
                .into_iter()
                .map(|v| v.as_str().to_string())
                .collect::<Vec<_>>()
                .join(" "),
        ))
    }

    fn fnc_try(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        if interp.error != ERROR_NOERROR {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let r = (*interp_mut).eval_value(&argv[0]);
            if (*interp_mut).error != ERROR_NOERROR {
                (*interp_mut).error = ERROR_NOERROR;
                if argv.len() > 1 {
                    (*interp_mut).eval_value(&argv[1])
                } else {
                    None
                }
            } else {
                r
            }
        }
    }

    fn fnc_error(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let msg = if argv.is_empty() {
            None
        } else {
            Some(argv[0].as_str())
        };
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            (*interp_mut).set_error(msg);
        }
        None
    }

    fn fnc_exit(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if let Some(cb) = &interp.callbacks.exit {
            let arg = argv.get(0).cloned();
            cb(interp, arg);
        }
        None
    }

    fn fnc_source(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.is_empty() {
            return None;
        }
        let path = argv[0].as_str();
        let buffer = if let Some(cb) = &interp.callbacks.source {
            cb(interp, path)
        } else if let Some(cb) = &interp.callbacks.read {
            cb(interp, path)
        } else {
            let mut f = match File::open(Path::new(path)) {
                Ok(f) => f,
                Err(_) => return None,
            };
            let mut buf = String::new();
            if f.read_to_string(&mut buf).is_err() {
                return None;
            }
            buf
        };
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe { (*interp_mut).parse(&buffer, 0, false) }
    }

    fn fnc_lmap(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        if argv.len() < 2 {
            return None;
        }
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            let list = (*interp_mut).subst_to_list(&argv[0]).unwrap_or_default();
            for (i, v) in argv.iter().enumerate().skip(1) {
                if let Some(lv) = list.get(i - 1) {
                    (*interp_mut).set_var(v.as_str(), lv.clone(), SetVarMode::Local);
                }
            }
            None
        }
    }

    fn fnc_rand(_interp: &LilInterpreter, _argv: &[LilValue]) -> LilValueOpt {
        let r: f64 = {
            use std::cell::Cell;
            thread_local!(static S: Cell<u64> = Cell::new(0x9E3779B97F4A7C15));
            S.with(|s| { let mut x = s.get(); x ^= x << 13; x ^= x >> 7; x ^= x << 17; s.set(x); (x >> 11) as f64 / (1u64 << 53) as f64 })
        };
        Some(LilValue::new(format!("{}", r)))
    }

    fn fnc_catcher(interp: &LilInterpreter, argv: &[LilValue]) -> LilValueOpt {
        let mut interp_mut = interp as *const _ as *mut LilInterpreter;
        unsafe {
            if argv.is_empty() {
                if let Some(c) = &(*interp_mut).catcher {
                    Some(LilValue::new(c.clone()))
                } else {
                    Some(LilValue::new(""))
                }
            } else {
                let catcher = argv[0].as_str();
                if catcher.is_empty() {
                    (*interp_mut).catcher = None;
                } else {
                    (*interp_mut).catcher = Some(catcher.to_string());
                }
                None
            }
        }
    }
}

// -------------- Variable set mode --------------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetVarMode {
    Local,
    Global,
    LocalNew,
    LocalOnly,
}

// -------------- Expression Evaluation Structures --------------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ExprType {
    Int,
    Float,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ExprError {
    NoError,
    SyntaxError,
    InvalidType,
    DivZero,
    InvalidExpression,
}

struct ExprEval<'a> {
    code: &'a [u8],
    len: usize,
    head: usize,
    ival: LilInt,
    dval: f64,
    ty: ExprType,
    error: ExprError,
}

impl<'a> ExprEval<'a> {
    fn new(s: &'a str) -> Self {
        ExprEval {
            code: s.as_bytes(),
            len: s.len(),
            head: 0,
            ival: 0,
            dval: 0.0,
            ty: ExprType::Int,
            error: ExprError::NoError,
        }
    }

    fn skip_spaces(&mut self) {
        while self.head < self.len && (self.code[self.head] as char).is_whitespace() {
            self.head += 1;
        }
    }

    fn numeric_element(&mut self) {
        let mut fpart: LilInt = 0;
        let mut fpartlen: LilInt = 1;
        self.ty = ExprType::Int;
        self.skip_spaces();
        self.ival = 0;
        self.dval = 0.0;
        while self.head < self.len {
            let ch = self.code[self.head] as char;
            if ch == '.' {
                if self.ty == ExprType::Float {
                    break;
                }
                self.ty = ExprType::Float;
                self.head += 1;
            } else if !ch.is_ascii_digit() {
                break;
            }
            if self.ty == ExprType::Int {
                self.ival = self.ival * 10 + (ch as u8 - b'0') as LilInt;
            } else {
                fpart = fpart * 10 + (ch as u8 - b'0') as LilInt;
                fpartlen *= 10;
            }
            self.head += 1;
        }
        if self.ty == ExprType::Float {
            self.dval = self.ival as f64 + fpart as f64 / fpartlen as f64;
        }
    }

    fn element(&mut self) {
        if self.head < self.len && (self.code[self.head] as char).is_ascii_digit() {
            self.numeric_element();
            return;
        }
        // strings evaluate as true when in conditionals
        self.ty = ExprType::Int;
        self.ival = 1;
        self.error = ExprError::InvalidExpression; // special flag
    }

    fn paren(&mut self) {
        self.skip_spaces();
        if self.head < self.len && self.code[self.head] as char == '(' {
            self.head += 1;
            self.expr();
            self.skip_spaces();
            if self.head < self.len && self.code[self.head] as char == ')' {
                self.head += 1;
            } else {
                self.error = ExprError::SyntaxError;
            }
        } else {
            self.element();
        }
    }

    fn unary(&mut self) {
        self.skip_spaces();
        if self.head < self.len
            && matches!(self.code[self.head] as char, '-' | '+' | '~' | '!')
        {
            let op = self.code[self.head] as char;
            self.head += 1;
            self.unary();
            if self.error != ExprError::NoError {
                return;
            }
            match op {
                '-' => match self.ty {
                    ExprType::Float => self.dval = -self.dval,
                    ExprType::Int => self.ival = -self.ival,
                },
                '+' => {}
                '~' => match self.ty {
                    ExprType::Float => {
                        self.ival = !(self.dval as LilInt);
                        self.ty = ExprType::Int;
                    }
                    ExprType::Int => self.ival = !self.ival,
                },
                '!' => match self.ty {
                    ExprType::Float => self.dval = if self.dval == 0.0 { 1.0 } else { 0.0 },
                    ExprType::Int => self.ival = if self.ival == 0 { 1 } else { 0 },
                },
                _ => {}
            }
        } else {
            self.paren();
        }
    }

    fn muldiv(&mut self) {
        self.unary();
        if self.error != ExprError::NoError {
            return;
        }
        self.skip_spaces();
        while self.head < self.len {
            let op = self.code[self.head] as char;
            if !matches!(op, '*' | '/' | '\\' | '%') {
                break;
            }
            let odval = self.dval;
            let oival = self.ival;
            self.head += 1;
            self.unary();
            if self.error != ExprError::NoError {
                return;
            }
            match op {
                '*' => match (self.ty, self.ty) {
                    (ExprType::Float, ExprType::Float) => self.dval *= odval,
                    (ExprType::Float, ExprType::Int) => self.dval *= oival as f64,
                    (ExprType::Int, ExprType::Float) => {
                        self.dval = self.dval * oival as f64;
                        self.ty = ExprType::Float;
                    }
                    (ExprType::Int, ExprType::Int) => self.ival *= oival,
                },
                '%' => {
                    let dv = match self.ty {
                        ExprType::Float => self.dval,
                        ExprType::Int => self.ival as f64,
                    };
                    let denom = match self.ty {
                        ExprType::Float => self.dval,
                        ExprType::Int => self.ival as f64,
                    };
                    if denom == 0.0 {
                        self.error = ExprError::DivZero;
                    } else {
                        self.dval = odval % denom;
                        self.ty = ExprType::Float;
                    }
                }
                '/' => {
                    let denom = match self.ty {
                        ExprType::Float => self.dval,
                        ExprType::Int => self.ival as f64,
                    };
                    if denom == 0.0 {
                        self.error = ExprError::DivZero;
                    } else {
                        self.dval = match self.ty {
                            ExprType::Float => odval / denom,
                            ExprType::Int => odval / denom,
                        };
                        self.ty = ExprType::Float;
                    }
                }
                '\\' => {
                    let denom = match self.ty {
                        ExprType::Float => self.dval,
                        ExprType::Int => self.ival as f64,
                    };
                    if denom == 0.0 {
                        self.error = ExprError::DivZero;
                    } else {
                        self.ival = (odval / denom) as LilInt;
                        self.ty = ExprType::Int;
                    }
                }
                _ => {}
            }
            self.skip_spaces();
        }
    }

    fn addsub(&mut self) {
        self.muldiv();
        self.skip_spaces();
        while self.head < self.len {
            let op = self.code[self.head] as char;
            if !matches!(op, '+' | '-') {
                break;
            }
            let odval = self.dval;
            let oival = self.ival;
            self.head += 1;
            self.muldiv();
            if self.error != ExprError::NoError {
                return;
            }
            match op {
                '+' => match (self.ty, self.ty) {
                    (ExprType::Float, ExprType::Float) => self.dval += odval,
                    (ExprType::Float, ExprType::Int) => self.dval += oival as f64,
                    (ExprType::Int, ExprType::Float) => {
                        self.dval = self.dval + oival as f64;
                        self.ty = ExprType::Float;
                    }
                    (ExprType::Int, ExprType::Int) => self.ival += oival,
                },
                '-' => match (self.ty, self.ty) {
                    (ExprType::Float, ExprType::Float) => self.dval = odval - self.dval,
                    (ExprType::Float, ExprType::Int) => self.dval = odval - self.ival as f64,
                    (ExprType::Int, ExprType::Float) => {
                        self.dval = oival as f64 - self.dval;
                        self.ty = ExprType::Float;
                    }
                    (ExprType::Int, ExprType::Int) => self.ival = oival - self.ival,
                },
                _ => {}
            }
            self.skip_spaces();
        }
    }

    fn shift(&mut self) {
        self.addsub();
        self.skip_spaces();
        while self.head + 1 < self.len {
            let op1 = self.code[self.head] as char;
            let op2 = self.code[self.head + 1] as char;
            if !(op1 == '<' && op2 == '<' || op1 == '>' && op2 == '>') {
                break;
            }
            self.head += 2;
            let odval = self.dval;
            let oival = self.ival;
            self.addsub();
            if self.error != ExprError::NoError {
                return;
            }
            match op1 {
                '<' => {
                    let shift = match self.ty {
                        ExprType::Float => self.dval as LilInt,
                        ExprType::Int => self.ival,
                    };
                    self.ival = match self.ty {
                        ExprType::Float => (odval as LilInt) << shift,
                        ExprType::Int => oival << shift,
                    };
                    self.ty = ExprType::Int;
                }
                '>' => {
                    let shift = match self.ty {
                        ExprType::Float => self.dval as LilInt,
                        ExprType::Int => self.ival,
                    };
                    self.ival = match self.ty {
                        ExprType::Float => (odval as LilInt) >> shift,
                        ExprType::Int => oival >> shift,
                    };
                    self.ty = ExprType::Int;
                }
                _ => {}
            }
            self.skip_spaces();
        }
    }

    fn compare(&mut self) {
        self.shift();
        self.skip_spaces();
        while self.head < self.len {
            let op = self.code[self.head] as char;
            let op2 = if self.head + 1 < self.len {
                self.code[self.head + 1] as char
            } else {
                '\0'
            };
            let kind = if op == '<' && op2 != '=' {
                1
            } else if op == '>' && op2 != '=' {
                2
            } else if op == '<' && op2 == '=' {
                3
            } else if op == '>' && op2 == '=' {
                4
            } else {
                break;
            };
            let odval = self.dval;
            let oival = self.ival;
            self.head += if kind > 2 { 2 } else { 1 };
            self.shift();
            if self.error != ExprError::NoError {
                return;
            }
            let rhs_f = match self.ty {
                ExprType::Float => self.dval,
                ExprType::Int => self.ival as f64,
            };
            let lhs_f = match self.ty {
                ExprType::Float => odval,
                ExprType::Int => oival as f64,
            };
            self.ival = match kind {
                1 => (lhs_f < rhs_f) as LilInt,
                2 => (lhs_f > rhs_f) as LilInt,
                3 => (lhs_f <= rhs_f) as LilInt,
                4 => (lhs_f >= rhs_f) as LilInt,
                _ => 0,
            };
            self.ty = ExprType::Int;
            self.skip_spaces();
        }
    }

    fn equals(&mut self) {
        self.compare();
        self.skip_spaces();
        while self.head + 1 < self.len {
            let op1 = self.code[self.head] as char;
            let op2 = self.code[self.head + 1] as char;
            if !(op1 == '=' && op2 == '=' || op1 == '!' && op2 == '=') {
                break;
            }
            let kind = if op1 == '=' { 1 } else { 2 };
            let odval = self.dval;
            let oival = self.ival;
            self.head += 2;
            self.compare();
            if self.error != ExprError::NoError {
                return;
            }
            let rhs_f = match self.ty {
                ExprType::Float => self.dval,
                ExprType::Int => self.ival as f64,
            };
            let lhs_f = match self.ty {
                ExprType::Float => odval,
                ExprType::Int => oival as f64,
            };
            self.ival = match kind {
                1 => (lhs_f == rhs_f) as LilInt,
                2 => (lhs_f != rhs_f) as LilInt,
                _ => 0,
            };
            self.ty = ExprType::Int;
            self.skip_spaces();
        }
    }

    fn bitand(&mut self) {
        self.equals();
        self.skip_spaces();
        while self.head < self.len && self.code[self.head] as char == '&' {
            self.head += 1;
            let odval = self.dval as LilInt;
            let oival = self.ival;
            self.equals();
            if self.error != ExprError::NoError {
                return;
            }
            let rhs = match self.ty {
                ExprType::Float => self.dval as LilInt,
                ExprType::Int => self.ival,
            };
            let lhs = if matches!(self.ty, ExprType::Float) {
                odval
            } else {
                oival
            };
            self.ival = lhs & rhs;
            self.ty = ExprType::Int;
            self.skip_spaces();
        }
    }

    fn bitor(&mut self) {
        self.bitand();
        self.skip_spaces();
        while self.head < self.len && self.code[self.head] as char == '|' {
            self.head += 1;
            let odval = self.dval as LilInt;
            let oival = self.ival;
            self.bitand();
            if self.error != ExprError::NoError {
                return;
            }
            let rhs = match self.ty {
                ExprType::Float => self.dval as LilInt,
                ExprType::Int => self.ival,
            };
            let lhs = if matches!(self.ty, ExprType::Float) {
                odval
            } else {
                oival
            };
            self.ival = lhs | rhs;
            self.ty = ExprType::Int;
            self.skip_spaces();
        }
    }

    fn logand(&mut self) {
        self.bitor();
        self.skip_spaces();
        while self.head + 1 < self.len
            && self.code[self.head] as char == '&'
            && self.code[self.head + 1] as char == '&'
        {
            self.head += 2;
            let od = match self.ty {
                ExprType::Float => self.dval != 0.0,
                ExprType::Int => self.ival != 0,
            };
            self.bitor();
            if self.error != ExprError::NoError {
                return;
            }
            let rhs = match self.ty {
                ExprType::Float => self.dval != 0.0,
                ExprType::Int => self.ival != 0,
            };
            self.ival = if od && rhs { 1 } else { 0 };
            self.ty = ExprType::Int;
            self.skip_spaces();
        }
    }

    fn logor(&mut self) {
        self.logand();
        self.skip_spaces();
        while self.head + 1 < self.len
            && self.code[self.head] as char == '|'
            && self.code[self.head + 1] as char == '|'
        {
            self.head += 2;
            let od = match self.ty {
                ExprType::Float => self.dval != 0.0,
                ExprType::Int => self.ival != 0,
            };
            self.logand();
            if self.error != ExprError::NoError {
                return;
            }
            let rhs = match self.ty {
                ExprType::Float => self.dval != 0.0,
                ExprType::Int => self.ival != 0,
            };
            self.ival = if od || rhs { 1 } else { 0 };
            self.ty = ExprType::Int;
            self.skip_spaces();
        }
    }

    fn expr(&mut self) {
        self.logor();
        if self.error == ExprError::InvalidExpression {
            self.error = ExprError::NoError;
            self.ival = 1;
            self.ty = ExprType::Int;
        }
    }
}
