"""The reporting script must use the same node identities as the matcher."""
import sys
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'scripts'))
sys.path.insert(0, str(ROOT / 'tools/stu_selector'))
from rq1_topology_resolution import topology_counts
from matcher import adjacency


class TopologyCountsTest(unittest.TestCase):
    def test_homonyms_and_projected_recursion(self):
        data = {
            'functions': [{'name': n} for n in ('swap', 'recur', 'crate::left::f', 'crate::right::f')],
            'raw_edges': [
                {'from': 'swap', 'to': '@nonlocal::core::mem::swap'},
                {'from': 'swap', 'to': '@nested::crate::swap'},
                {'from': 'recur', 'to': 'recur'},
                {'from': 'recur', 'to': 'recur'},
                {'from': 'crate::left::f', 'to': 'crate::right::f'},
                {'from': 'crate::right::f', 'to': '@nonlocal::foreign::f'},
            ],
            'indirect_calls': [{'from': 'recur', 'kind': 'call_unresolved'}],
        }
        counts = topology_counts(data)
        self.assertEqual(counts['local_sites'], 3)
        self.assertEqual(counts['nonlocal_sites'], 3)
        self.assertEqual(counts['unique_local_edges'], 2)
        self.assertEqual(counts['unique_local_noself'], 1)
        out, _ = adjacency(data)
        self.assertEqual(counts['unique_local_edges'], sum(map(len, out.values())))
        self.assertEqual(out['swap'], set())

    def test_empty_graph(self):
        counts = topology_counts({'functions': [], 'raw_edges': [], 'indirect_calls': []})
        self.assertEqual(counts['edges_per_fn'], 0)
        self.assertEqual(counts['unresolved_rate'], 0)


if __name__ == '__main__':
    unittest.main()
