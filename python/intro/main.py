from datetime import datetime

# --- Classes and helper funtions ---


class Person:
    def __init__(self, first_name: str, age: int) -> None:
        self.first_name = first_name
        if age > 0 and isinstance(age, int):
            self.age = age
        else:
            raise ValueError("Age must be a positive integer")

    def __str__(self) -> str:
        return f"{self.first_name} is {self.age} years old"

    def __repr__(self) -> str:
        return f"Person: {self.first_name}, {self.age}"


def approx_year_of_birth(person: Person) -> int:
    birth_year_approx = datetime.today().year - person.age
    return birth_year_approx


# --- Run functions ---


def run1() -> None:
    """
    1. Protocols
    """
    person = Person("Megan", 28)
    print(person)
    print(repr(person))
    """
    Megan is 28 years old
    Person: Megan, 28
    """


def run2() -> None:
    """
    2. Enumerate
    """
    persons = [Person("James", 33), Person("Salima", 31)]
    for i, person in enumerate(persons):
        print(f"{i}: {str(person)}")
    """
    0: James is 33 years old
    1: Salima is 31 years old
    """


def run3() -> None:
    """
    3. Zip
    """
    names = ["Alice", "Charlie"]
    ages = [24, 45]
    persons = []
    for name, age in zip(names, ages):
        person = Person(name, age)
        persons.append(person)
    print(f"{repr(persons)}")
    """
    [Person: Alice, 24, Person: Charlie, 45]
    """


def run4() -> None:
    """
    4. First-class functions
    """
    person = Person("Khalil", 50)
    birth_year_approx = approx_year_of_birth(person)
    print(f"{str(person)}. They are estimated to be born in {birth_year_approx}")
    """
    Khalil is 50 years old. They are estimated to be born in 1974
    """


def run5() -> None:
    """
    5. Anonymous functions
    """
    persons = [Person("Aiko", 41), Person("Rohan", 18)]
    sorted_by_age = sorted(persons, key=lambda person: person.age)
    youngest_person = sorted_by_age[0]
    print(f"{youngest_person.first_name} is the youngest person at {youngest_person.age} years old")
    """
    Rohan is the youngest person at 18 years old
    """


def run6() -> None:
    """
    6. Single line if statements
    """
    persons = [Person("Josephine", 20), Person("Wesley", 31)]
    for person in persons:
        # Check if person is born in a leap year using simplistic leap year logic
        birth_year = approx_year_of_birth(person)
        person_is_born_in_leap_year = True if birth_year % 4 == 0 else False
        print(f"{person}. Born in leap year?: {person_is_born_in_leap_year}")
    """
    Josephine is 20 years old. Born in leap year?: True
    Will is 31 years old. Born in leap year?: False
    """


def run7() -> None:
    """
    7. List comprehensions
    """
    persons = [Person("Adebayo", 29), Person("Ibrahim", 26)]
    birth_years = [approx_year_of_birth(person) for person in persons]
    person_names = [person.first_name for person in persons]
    # Combine with zip
    persons_and_birth_years = list(zip(person_names, birth_years))
    for person in persons_and_birth_years:
        print(f"{person[0]} is estimated to be born in {person[1]}")
    """
    Adebayo is estimated to be born in 1995
    Ibrahim is estimated to be born in 1998
    """


def main() -> None:
    run1()
    run2()
    run3()
    run4()
    run5()
    run6()
    run7()


if __name__ == "__main__":
    main()
