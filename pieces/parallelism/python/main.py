import argparse
import csv
import re
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path
from typing import Any, Iterator

JsonBlob = dict[str, Any]
# Set large field size limit for this script
csv.field_size_limit(50_000_000)


def load_csv(input_file: Path) -> list[JsonBlob]:
    records = None
    # Assumes each file is small enough (<100K rows) that we can load it all into memory at once
    with open(input_file) as f:
        next(f)  # Skip header
        reader = csv.reader(f)
        records = [parse_record(item) for item in reader]
    return records


def parse_record(item: list[Any]) -> JsonBlob:
    return {
        "id": item[1],
        "publication": item[3],
        "author": item[4],
        "date": item[5],
        "content": item[9],
    }


class BatchProcessor:
    def __init__(self, batch_size: int):
        self.batch_size = batch_size
        self.pattern1 = re.compile(r"[’']s")
        self.pattern2 = re.compile(r"[’']d")
        self.pattern3 = re.compile(r"[’']ll")

    def _create_batches(self, data: list[JsonBlob]) -> Iterator[list[JsonBlob]]:
        """Yield batches of data of the specified size"""
        for i in range(0, len(data), self.batch_size):
            yield data[i : i + self.batch_size]

    def _clean_text(self, text: str) -> str:
        text_lower = text.lower()
        suffix_mapping = {
            "s": " is",
            "d": " had",
            "ll": " will",
        }
        # Replace contractions with full words
        formatted_text = re.sub(r"([’'])(s|d|ll)", lambda x: suffix_mapping[x.group(2)], text_lower)
        # Remove non-alphabetic characters
        result = re.sub(r"[^a-zA-Z\s]", "", formatted_text)
        return result

    def calculate_counts(self, data: JsonBlob) -> JsonBlob:
        text = data["content"]
        result = self._clean_text(text)
        tokens = result.split()
        data["num_male_pronouns"], data["num_female_pronouns"] = count_gendered_pronouns(tokens)
        data.pop("content")
        return data

    def process_batches(self, data: list[JsonBlob]) -> list[JsonBlob]:
        with ProcessPoolExecutor(max_workers=8) as executor:
            batches = list(self._create_batches(data))
            # Process batches in parallel
            results = []
            for batch in batches:
                batch_results = list(executor.map(self.calculate_counts, batch))
                results.extend(batch_results)
            return results


def count_gendered_pronouns(tokens: list[str]) -> tuple[int, int]:
    num_male_pronouns = sum(1 for token in tokens if token in ["he", "him", "his"])
    num_female_pronouns = sum(1 for token in tokens if token in ["she", "her", "hers"])
    return num_male_pronouns, num_female_pronouns


def write_results(data: list[JsonBlob], file_path: Path, file_name: str) -> None:
    output_path = file_path / file_name
    fieldnames = ["id", "publication", "author", "date", "num_male_pronouns", "num_female_pronouns"]
    with open(output_path, "w") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(data)


def main(file_path: Path, batch_size: int) -> None:
    # Get all .csv files in the directory
    files = [
        Path(f"../data/{file}") for file in ("articles1.csv", "articles2.csv", "articles3.csv")
    ]
    processor = BatchProcessor(batch_size)
    for input_file in files:
        records = load_csv(input_file)
        results = processor.process_batches(records)
        output_file = input_file.name.replace(".csv", "_processed.csv")
        write_results(results, file_path, output_file)
        print(f"Results for {input_file} written to: {file_path / output_file}")


if __name__ == "__main__":
    # fmt: off
    parser = argparse.ArgumentParser(description="Count gendered pronouns in a dataset")
    parser.add_argument("--batch_size", "-b", type=int, default=250,help="Number or records for each batch processed in parallel")
    parser.add_argument("--file_path", "-p", type=str, default="../data", help="Path to input files")
    parser.add_argument("--num_workers", "-n", type=int, default=4, help="Maximum number of worker processes")
    args = parser.parse_args()
    # fmt: on

    file_path = Path(args.file_path)
    main(file_path, args.batch_size)
