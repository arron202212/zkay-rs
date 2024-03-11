// from parameterized::parameterized_class

// from zkay.examples.scenario::Scenario
// from zkay.examples.example_scenarios::all_scenarios
// from zkay.examples.examples::all_examples, Example
// from zkay.tests.zkay_unit_test::ZkayTestCase


// class TestExamples(ZkayTestCase):
//     name: str = None
//     example: Example = None


// class TestScenarios(ZkayTestCase):
//     name: str = None
//     scenario: Scenario = None


// @parameterized_class(('name', 'example'), all_examples)
// class TestExamplesFunctions(TestExamples):

//     def test_file_location(self):
//         self.assertIsNotNone(self.example.file_location)

//     def test_code(self):
//         self.assertIsNotNone(self.example.code())

//     def test_stream(self):
//         self.assertIsNotNone(self.example.stream())

//     def test_name(self):
//         self.assertIsNotNone(self.example.name())


// @parameterized_class(('name', 'scenario'), all_scenarios)
// class TestScenariosFunctions(TestScenarios):

//     def test_file_location(self):
//         self.assertIsNotNone(self.scenario.file_location)

//     def test_code(self):
//         self.assertIsNotNone(self.scenario.code())

//     def test_users(self):
//         self.assertIsNotNone(self.scenario.users())

//     def test_deployment_transaction(self):
//         self.assertIsNotNone(self.scenario.deployment_transaction())

//     def test_transactions(self):
//         self.assertGreater(len(self.scenario.transactions_and_assertions()), 0)
