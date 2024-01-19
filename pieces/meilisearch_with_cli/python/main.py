import asyncio
from pathlib import Path
from typing import Any, List

import srsly
from meilisearch_python_sdk import AsyncClient, Client
from meilisearch_python_sdk.models.settings import MeilisearchSettings
from rich.console import Console
from typer import Argument, Exit, Option, Typer, echo

app = Typer()

__version__ = "0.1.0"

MEILISEARCH_URL = "http://127.0.0.1:7700"
MEILISEARCH_API_KEY = "apiKey"

console = Console()


async def preform_indexing(data: List[dict[str, Any]], index_name: str, wait: bool) -> None:
    async with AsyncClient(MEILISEARCH_URL, MEILISEARCH_API_KEY) as client:
        index = client.index(index_name)
        tasks = await index.add_documents_in_batches(data)
        if wait:
            waits = [client.wait_for_task(task.task_uid, timeout_in_ms=None) for task in tasks]
            await asyncio.gather(*waits)


@app.command()
def create_index(
    index_name: str = Option("wine", "-i", "--index-name", help="The name to use for the index"),
) -> None:
    client = Client(MEILISEARCH_URL, MEILISEARCH_API_KEY)
    client.create_index(
        index_name,
        primary_key="id",
        settings=MeilisearchSettings(
            ranking_rules=["sort", "words", "typo", "proximity", "attribute", "exactness"],
            sortable_attributes=["title", "country"],
            searchable_attributes=[
                "title",
                "description",
                "taster_name",
                "designation",
                "variety",
                "province",
                "country",
                "winery",
            ],
        ),
    )

    console.print(f"Index {index_name} successfully created", style="green")


@app.command()
def index_data(
    data_path: Path = Option(
        "../data/winemag-data-130k-v2.jsonl.gz",
        "-d",
        "--data-path",
        exists=True,
        file_okay=True,
        dir_okay=False,
        help="Path to the data file",
    ),
    index_name: str = Option("wine", "-i", "--index-name", help="The name to use for the index"),
    wait: bool = Option(False, "-w", "--wait", help="Wait for the data to finish indexing"),
) -> None:
    if data_path:
        data = srsly.read_gzip_jsonl(data_path)
    else:
        data = srsly.read_gzip_jsonl(Path("../data/winemag-data-130k-v2.jsonl.gz"))
    with console.status("Indexing data..."):
        asyncio.run(preform_indexing(list(data), index_name, wait))

    console.print("Indexing complete", style="green")


@app.command()
def search(
    query: str = Argument(..., help="The search to preform"),
    limit: int = Option(20, "-l", "--limit", help="Limit the number of search results"),
    sort: List[str] = Option(None, "-s", "--sort", help="Sort order for the results"),
    index_name: str = Option("wine", "-i", "--index-name", help="The name to use for the index"),
) -> None:
    client = Client(MEILISEARCH_URL, MEILISEARCH_API_KEY)
    index = client.index(index_name)
    results = index.search(query, limit=limit, sort=sort)
    console.print(results.hits)


@app.callback(invoke_without_command=True)
def main(
    version: bool = Option(
        False, "--version", "-v", is_eager=True, help="Show the instealled version"
    ),
) -> None:
    if version:
        echo(__version__)
        raise Exit()


if __name__ == "__main__":
    app()
