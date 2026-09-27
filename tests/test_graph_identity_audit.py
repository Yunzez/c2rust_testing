import sys
from pathlib import Path
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'scripts'))
from rq1_graph_identity_audit import evaluate_pairs


class LabelIdentityTest(unittest.TestCase):
    def test_duplicate_leaf_is_unadjudicated_not_correct_or_wrong(self):
        data = {'functions': [{'name': 'left::same'}, {'name': 'right::same'}]}
        result = evaluate_pairs([('c', 'left::same', .8, .1)], data, {'c': 'same'})
        self.assertEqual(result['correct'], 0)
        self.assertEqual(result['wrong'], {})
        self.assertEqual(result['unadjudicated'], {'c': 'left::same'})
        self.assertEqual((result['precision'], result['precision_upper']), (0, 1))

    def test_nonpair_prediction_is_still_a_false_positive(self):
        data = {'functions': [{'name': 'a'}, {'name': 'b'}]}
        result = evaluate_pairs([('a', 'a', .8, .1), ('stub', 'b', .9, .1)], data, {'a': 'a'})
        self.assertEqual(result['correct'], 1)
        self.assertEqual(result['accepted'], 2)
        self.assertEqual(result['precision'], .5)
        self.assertEqual(result['wrong'], {'stub': 'b'})


if __name__ == '__main__':
    unittest.main()
