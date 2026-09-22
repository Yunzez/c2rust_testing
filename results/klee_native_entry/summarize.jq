def ineligible:
  (.status | startswith("ineligible_")) or .status == "input_bound_excluded";

def count_status($name):
  [.cases[] | select(.status == $name)] | length;

{
  schema: "klee-native-entry-summary-v1",
  total_cases: (.cases | length),
  qualified_cases: ([.cases[] | select(ineligible | not)] | length),
  lifted_witnesses: ([.cases[] | select(.status | startswith("lifted_"))] | length),
  same_defect_process_exposures: count_status("lifted_exposed"),
  lifted_blocked_by_prior_divergence: count_status("lifted_blocked"),
  lifted_not_exposed: count_status("lifted_not_exposed"),
  pending_or_running: ([.cases[] | select(
    (.status == "primary_running") or
    (.status == "extension_running") or
    (.status == "eligibility_audit_pending"))] | length),
  bounded_misses: ([.cases[] | select(
    (.status == "not_lifted_2h") or
    (.status == "not_lifted_8h"))] | length),
  exclusions: {
    no_native_entry: count_status("ineligible_no_native_entry"),
    no_native_path: count_status("ineligible_no_native_path"),
    unqualified_c_reference: count_status("ineligible_c_reference"),
    input_bound: count_status("input_bound_excluded")
  },
  by_status: (
    [.cases[].status]
    | sort
    | group_by(.)
    | map({key: .[0], value: length})
    | from_entries
  )
}
