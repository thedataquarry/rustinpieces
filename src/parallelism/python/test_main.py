from pathlib import Path

import pytest
from main import BatchProcessor, count_gendered_pronouns, load_csv


@pytest.fixture
def processor():
    return BatchProcessor(10)


@pytest.fixture
def data():
    path_to_file = Path("../data/articles1.csv")
    data = load_csv(path_to_file)[:10]
    return data


def test_record(data):
    assert len(data) > 0
    assert data[0]["id"] is not None
    assert data[0]["publication"] is not None
    assert data[0]["author"] is not None
    assert data[0]["date"] is not None
    assert data[0]["content"] is not None


def test_count_gendered_pronouns(processor):
    text = "He's he’ll he’d HE she's she'll She’d SHE random text here"
    clean_text = processor._clean_text(text)
    tokens = clean_text.split()
    gendered_pronouns = count_gendered_pronouns(tokens)
    assert gendered_pronouns == (4, 4)


def test_clean_text(processor):
    text = "He's he’s he’ll she'll he’d She'd HE SHE"
    result = processor._clean_text(text)
    assert result == "he is he is he will she will he had she had he she"
    assert result


def test_calculate_counts(processor):
    data = dict()
    data["content"] = """
        He'll surely visit. Her dog's going to be there, she promised him many times.
        His apartment is close to hers, and she'd often visit him.
    """
    result = processor.calculate_counts(data)
    assert result["num_male_pronouns"] == 4
    assert result["num_female_pronouns"] == 4


def test_process_batches(data, processor):
    results = processor.process_batches(data)
    assert len(results) == len(data)
