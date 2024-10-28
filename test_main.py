import unittest
import os
import pandas as pd
from main import process_customer_data, generate_markdown_report

class TestCustomerProcessing(unittest.TestCase):

    def tearDown(self):
        """Cleanup files after tests."""
        for file in ["test_input.csv", "test_output.csv", "Python_Performance.md"]:
            if os.path.exists(file):
                os.remove(file)

    def test_data_processing_creates_correct_output(self):
        """Test that the data processing function generates the correct output."""
        input_file = "test_input.csv"
        output_file = "test_output.csv"

        # Create a simple test dataset
        data = {
            "user_id": [1, 2, 3],
            "purchase_amount": [450, 350, 500],
            "loyalty_score": [8.0, 6.0, 9.0],
            "annual_income": [50000, 40000, 60000],
            "purchase_frequency": [12, 10, 15],
        }
        df = pd.DataFrame(data)
        df.to_csv(input_file, index=False)

        # Run the function
        total_income, avg_income, total_frequency, avg_frequency = process_customer_data(input_file, output_file)

        # Assertions
        self.assertTrue(os.path.exists(output_file))
        self.assertEqual(total_income, 110000)  # 50000 + 60000
        self.assertAlmostEqual(avg_income, 55000.0)
        self.assertEqual(total_frequency, 27)  # 12 + 15
        self.assertAlmostEqual(avg_frequency, 13.5)

    def test_markdown_report_contains_correct_metrics(self):
        """Test that the Markdown report is correctly generated with valid content."""
        markdown_file = "Python_Performance.md"

        # Generate a sample Markdown file
        generate_markdown_report(
            exec_time=0.5,
            mem_usage=[40.5, 42.0, 41.5],
            total_income=110000,
            avg_income=55000.0,
            total_frequency=27,
            avg_frequency=13.5,
            output_file="test_output.csv"
        )

        # Assertions
        self.assertTrue(os.path.exists(markdown_file))
        with open(markdown_file, "r") as f:
            content = f.read()
            self.assertIn("**Execution Time:** 0.50 seconds", content)
            self.assertIn("**Total Annual Income:** 110000", content)
            self.assertIn("The filtered and transformed data has been saved to `test_output.csv`.", content)


if __name__ == "__main__":
    unittest.main()
