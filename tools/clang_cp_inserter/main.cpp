#include <clang/AST/AST.h>
#include <clang/ASTMatchers/ASTMatchFinder.h>
#include <clang/Frontend/FrontendActions.h>
#include <clang/Rewrite/Core/Rewriter.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Tooling.h>
#include <llvm/Support/CommandLine.h>
#include <llvm/Support/raw_ostream.h>

using namespace clang;
using namespace clang::ast_matchers;
using namespace clang::tooling;

static llvm::cl::OptionCategory CpInserterCategory("cp-inserter options");
static llvm::cl::opt<std::string> FunctionList(
    "functions",
    llvm::cl::desc("Comma-separated list of function names to instrument"),
    llvm::cl::value_desc("f1,f2,f3"),
    llvm::cl::Required,
    llvm::cl::cat(CpInserterCategory));

static llvm::cl::opt<bool> InPlace(
    "in-place",
    llvm::cl::desc("Rewrite files in place"),
    llvm::cl::init(false),
    llvm::cl::cat(CpInserterCategory));

static llvm::cl::opt<std::string> CheckpointSymbol(
    "cp-symbol",
    llvm::cl::desc("Checkpoint function symbol (default: cp)"),
    llvm::cl::init("cp"),
    llvm::cl::cat(CpInserterCategory));

static bool isTargetFunction(const FunctionDecl *FD) {
    if (!FD || !FD->getIdentifier()) return false;
    std::string name = FD->getName().str();
    std::string list = FunctionList;
    size_t start = 0;
    while (start <= list.size()) {
        size_t end = list.find(',', start);
        if (end == std::string::npos) end = list.size();
        std::string item = list.substr(start, end - start);
        if (item == name) return true;
        start = end + 1;
    }
    return false;
}

class CpInserter : public MatchFinder::MatchCallback {
public:
    explicit CpInserter(Rewriter &R) : R(R) {}

    void run(const MatchFinder::MatchResult &Result) override {
        if (const auto *FD = Result.Nodes.getNodeAs<FunctionDecl>("func")) {
            if (!FD->hasBody()) return;
            if (!isTargetFunction(FD)) return;

            const auto *Body = dyn_cast<CompoundStmt>(FD->getBody());
            if (!Body) return;

            SourceLocation Loc = Body->getLBracLoc().getLocWithOffset(1);
            std::string entry = "\n    " + CheckpointSymbol + "(\"entry\", NULL, 0);";
            R.InsertText(Loc, entry, true, true);
        }

        if (const auto *RS = Result.Nodes.getNodeAs<ReturnStmt>("ret")) {
            const auto *FD = Result.Nodes.getNodeAs<FunctionDecl>("retfunc");
            if (!isTargetFunction(FD)) return;

            SourceLocation Loc = RS->getBeginLoc();
            std::string exit = CheckpointSymbol + "(\"exit\", NULL, 0);\n    ";
            R.InsertText(Loc, exit, true, true);
        }
    }

private:
    Rewriter &R;
};

class CpASTConsumer : public ASTConsumer {
public:
    explicit CpASTConsumer(Rewriter &R) : Handler(R) {
        Matcher.addMatcher(functionDecl(isDefinition()).bind("func"), &Handler);
        Matcher.addMatcher(returnStmt(hasAncestor(functionDecl().bind("retfunc"))).bind("ret"), &Handler);
    }

    void HandleTranslationUnit(ASTContext &Context) override {
        Matcher.matchAST(Context);
    }

private:
    CpInserter Handler;
    MatchFinder Matcher;
};

class CpFrontendAction : public ASTFrontendAction {
public:
    CpFrontendAction() = default;

    std::unique_ptr<ASTConsumer> CreateASTConsumer(CompilerInstance &CI,
                                                   StringRef InFile) override {
        R.setSourceMgr(CI.getSourceManager(), CI.getLangOpts());
        return std::make_unique<CpASTConsumer>(R);
    }

    void EndSourceFileAction() override {
        auto &SM = R.getSourceMgr();
        FileID FID = SM.getMainFileID();
        if (InPlace) {
            R.overwriteChangedFiles();
        } else {
            if (R.getRewriteBufferFor(FID)) {
                R.getEditBuffer(FID).write(llvm::outs());
            } else {
                llvm::outs() << SM.getBufferData(FID);
            }
        }
    }

private:
    Rewriter R;
};

int main(int argc, const char **argv) {
    auto ExpectedParser = CommonOptionsParser::create(argc, argv, CpInserterCategory);
    if (!ExpectedParser) {
        llvm::errs() << ExpectedParser.takeError();
        return 1;
    }
    CommonOptionsParser &OptionsParser = ExpectedParser.get();

    ClangTool Tool(OptionsParser.getCompilations(), OptionsParser.getSourcePathList());
    return Tool.run(newFrontendActionFactory<CpFrontendAction>().get());
}
