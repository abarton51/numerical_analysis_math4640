import argparse
import importlib

def run_question(question_module):
    try:
        # Dynamically import the question module (e.g., q1.py)
        question = importlib.import_module(question_module)
        question.main()  # Assuming each question file has a main function
    except ModuleNotFoundError:
        print(f"Error: Could not find the module {question_module}")

def main():
    parser = argparse.ArgumentParser(description='Run Numerical Analysis Questions')
    parser.add_argument('-q', '--question', type=str, required=True, help='The question to run (e.g., q1, q2, etc.)')
    args = parser.parse_args()

    # Run the desired question module
    run_question(args.question)

if __name__ == "__main__":
    main()

