
class User:
    def __init__(self, name):
        self.name = name

    def display_name(self):
        print("Hello, " + self.name)

    def set_name(self, name):
        self.name = name


def display_hello_world():
    print("Hello, world!")


x = 5

def main():
    display_hello_world()


if __name__ == "__main__":
    main()