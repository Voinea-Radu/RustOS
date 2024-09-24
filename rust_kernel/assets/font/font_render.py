from PIL import Image, ImageDraw, ImageFont


def generate_font_image(font):
    text = ""
    multiplier = 1

    for i in range(33, 126 + 1):
        text += chr(i)

    font = ImageFont.truetype(f"{font}.ttf", 12 * multiplier)
    _, _, width, height = font.getbbox(text)

    image = Image.new("RGB", (int(width), int(height)), color="white")
    draw = ImageDraw.Draw(image)

    draw.text((0, 0), text, (0, 0, 0), font=font)
    image.save(f"{font}.ppm")


def main():
    generate_font_image("noto_sans_mono")


if __name__ == "__main__":
    main()
