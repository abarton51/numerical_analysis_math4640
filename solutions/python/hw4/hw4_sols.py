import argparse
import importlib

def run_question(question_module):
    try:
        question = importlib.import_module(question_module)
        question.main()  # Assuming each question file has a main function
    except ModuleNotFoundError:
        print(f"Error: Could not find the module {question_module}")

def main():
    parser = argparse.ArgumentParser(description='Run Numerical Analysis Questions')
    parser.add_argument('-q', '--question', type=str, required=True, nargs='+', 
                        help='The questions to run (e.g., q1, q2, etc.)')
    args = parser.parse_args()

    print(f"Running the following question modules: {args.question}")

    for question in args.question:
        run_question(question)

if __name__ == "__main__":
    main()

