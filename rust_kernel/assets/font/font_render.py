from PIL import Image, ImageDraw, ImageFont


def generate_font_image(font_name, font_size):
    text = ""

    for i in range(33, 126 + 1):
        text += chr(i)

    font = ImageFont.truetype(f"{font_name}.ttf", font_size)
    _, _, width, height = font.getbbox(text)

    image = Image.new("RGB", (int(width), int(height)), color="white")
    draw = ImageDraw.Draw(image)

    draw.text((0, 0), text, (0, 0, 0), font=font)
    image.save(f"{font_name}.ppm")


def main():
    generate_font_image("noto_sans_mono", 12)


if __name__ == "__main__":
    main()
