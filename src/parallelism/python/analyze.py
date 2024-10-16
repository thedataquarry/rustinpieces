import polars as pl


def get_result() -> pl.DataFrame:
    articles1 = pl.read_csv("../data/articles1_processed.csv")
    articles2 = pl.read_csv("../data/articles2_processed.csv")
    articles3 = pl.read_csv("../data/articles3_processed.csv")
    # Combine the data into a single DataFrame
    result = pl.concat([articles1, articles2, articles3]).unique(subset=["id"]).sort("id")
    print(f"Number of articles: {result.height}")
    return result


def get_article_count_by_year(df: pl.DataFrame) -> pl.DataFrame:
    # Parse dates
    result = df.drop_nulls(subset="date").with_columns(
        pl.col("date").str.to_date("%Y-%m-%d", strict=False),
    )
    # Drop nulls and extract year
    result = result.filter(pl.col("date").is_not_null()).with_columns(
        pl.col("date").dt.year().alias("year"),
    )
    # Group by year and count
    result_by_year = result.group_by("year").len().sort("year", descending=True).head(5)
    return result_by_year


def get_pub_with_most_female_pronouns(df: pl.DataFrame) -> pl.DataFrame:
    result = (
        df.group_by("publication")
        .mean()
        .select("publication", "num_male_pronouns", "num_female_pronouns")
        .sort("num_female_pronouns", descending=True)
    )
    return result


def main() -> None:
    df = get_result()
    print(get_article_count_by_year(df))
    print(get_pub_with_most_female_pronouns(df))


if __name__ == "__main__":
    main()
