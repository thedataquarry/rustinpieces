from datetime import datetime

# --- Classes and helper funtions ---


class Person:
    def __init__(self, name: str, age: int) -> None:
        self.name = name
        if age > 0 and isinstance(age, int):
            self.age = age
        else:
            raise ValueError("Age must be a positive integer")

    def __str__(self) -> str:
        return f"{self.name} is {self.age} years old"

    def __repr__(self) -> str:
        return f"Person: {self.name}, {self.age}"


def approx_year_of_birth(person: Person) -> int:
    birth_year_approx = datetime.now().year - person.age
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
    Tuple unpacking
    """
    sorted_ages = (18, 41, 65)
    youngest, _, oldest = sorted_ages
    print(f"Youngest age: {youngest}, oldest age: {oldest}")
    print(f"Middle age: {sorted_ages[1]}")
    """
    Youngest age: 18, oldest age: 65
    Middle age: 41
    """


def run5() -> None:
    """
    5. Anonymous functions (lambdas)
    """
    persons = [Person("Aiko", 41), Person("Rohan", 18)]
    sorted_by_age = sorted(persons, key=lambda person: person.age)
    youngest_person = sorted_by_age[0]
    print(
        f"{youngest_person.name} is the youngest person at {youngest_person.age} years old"
    )
    """
    Rohan is the youngest person at 18 years old
    """


def run6() -> None:
    """
    6. Single-line if-else
    """
    persons = [Person("Josephine", 20), Person("Wesley", 31)]
    for person in persons:
        # Check if person is born in a leap year using simplistic leap year logic
        birth_year = approx_year_of_birth(person)
        person_is_born_in_leap_year = True if birth_year % 4 == 0 else False
        print(f"{person}. Born in a leap year?: {person_is_born_in_leap_year}")
    """
    Josephine is 20 years old. Born in leap year?: True
    Will is 31 years old. Born in leap year?: False
    """


def run7() -> None:
    """
    7. List comprehensions
    """
    persons = [Person("Issa", 39), Person("Ibrahim", 26)]
    persons_born_after_1995 = [
        (person.name, person.age)
        for person in persons
        if approx_year_of_birth(person) > 1995
    ]
    print(f"Persons born after 1995: {persons_born_after_1995}")
    """
    Persons born after 1995: [('Ibrahim', 26)]
    """


def run8() -> None:
    """
    8. dicts
    """
    processors = {
        "13900KS": "Intel Core i9",
        "13700K": "Intel Core i7",
        "13600K": "Intel Core i5",
        "1800X": "AMD Ryzen 7",
        "1600X": "AMD Ryzen 5",
        "1300X": "AMD Ryzen 3",
    }

    print(f"All processors: {processors}")
    print(f"Processor 13600K information by key: {processors['13600K']}")
    for k, v in processors.items():
        if v.lower() == "amd ryzen 3":
            print(f"Processor AMD Ryzen 3 information by value: {k}: {v}")
            break
    """
    All processors: {'13900KS': 'Intel Core i9', '13700K': 'Intel Core i7', '13600K': 'Intel Core i5', '1800X': 'AMD Ryzen 7', '1600X': 'AMD Ryzen 5', '1300X': 'AMD Ryzen 3'}
    Processor 13600K information by key: Intel Core i5
    Processor AMD Ryzen 3 information by value: 1300X: AMD Ryzen 3
    """


def run9() -> None:
    processors = {
        "Intel Core i9",
        "Intel Core i7",
        "Intel Core i5",
        "AMD Ryzen 7",
        "AMD Ryzen 5",
        "AMD Ryzen 3",
    }
    processors.add("Intel Core i7")
    processors.add("AMD Ryzen 5")

    print(processors)
    for processor in processors:
        if processor.lower() == "amd ryzen 3":
            print(processor)
            break
    """
    {'Intel Core i5', 'AMD Ryzen 3', 'Intel Core i7', 'AMD Ryzen 5', 'Intel Core i9', 'AMD Ryzen 7'}
    AMD Ryzen 3
    """


def main() -> None:
    run1()
    run2()
    run3()
    run4()
    run5()
    run6()
    run7()
    run8()
    run9()


if __name__ == "__main__":
    main()
