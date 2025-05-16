from abc import ABC, abstractmethod

class Color(ABC):
    @abstractmethod
    def fill(self) -> str:
        pass

class Red(Color):
    def fill(self) -> str:
        return "red"
    
class Blue(Color):
    def fill(self) -> str:
        return "blue"
    
class Shape(ABC):
    def __init__(self, color: Color):
        self.color = color

    @abstractmethod
    def draw(self) -> str:
        pass

class Circle(Shape):
    def draw(self) -> str:
        return f"Drawing a {self.color.fill()} circle"
    
class Square(Shape):
    def draw(self) -> str:
        return f"Drawing a {self.color.fill()} square"
    
if __name__ == "__main__":
    red = Red()
    blue = Blue()

    red_circle = Circle(red)
    blue_square = Square(blue)

    print(red_circle.draw())
    print(blue_square.draw())