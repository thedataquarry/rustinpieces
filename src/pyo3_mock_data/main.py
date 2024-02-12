import argparse
from pathlib import Path

from pyo3_mock_data import generate_mock_persons


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--num", "-n", type=int, default=10, help="Number of mock profiles to generate"
    )
    args = parser.parse_args()
    generate_mock_persons(Path("data/worldcities.csv"), args.num)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
