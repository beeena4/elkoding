class Game:
    def __init__(self):
        self.is_running = True

    def run(self):
        while self.is_running:
            self.update()
            self.draw()

    def update(self):
        pass  # Update game state

    def draw(self):
        pass  # Render game elements