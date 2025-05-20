def check_collision(rect1, rect2):
    return rect1.colliderect(rect2)

def draw_text(surface, text, position, font, color):
    text_surface = font.render(text, True, color)
    surface.blit(text_surface, position)

def reset_score():
    return 0

def update_score(score, increment=1):
    return score + increment