from typing import Protocol, List, Union


class ObserverProtocol(Protocol):
    def notify(self, value: Union[str, int]) -> None:
        pass


class Counter:
    def __init__(self) -> None:
        self.value: int = 0
        self.observers: List[ObserverProtocol] = []

    def register(self, observer: ObserverProtocol) -> None:
        self.observers.append(observer)

    def run(self) -> None:
        for observer in self.observers:
            self.value += 1
            observer.notify(self.value)


class Foo:
    def notify(self, value: Union[str, int]) -> None:
        print(f"Value is: {value}")


def main() -> None:
    observer = Foo()

    counter_1 = Counter()
    counter_2 = Counter()
    counter_3 = Counter()

    counter_1.register(observer)
    counter_2.register(observer)
    counter_3.register(observer)

    counter_1.run()
    counter_2.run()
    counter_3.run()


if __name__ == "__main__":
    main()
