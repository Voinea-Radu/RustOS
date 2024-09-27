import os
import sys
from PIL import Image, ImageDraw, ImageFont
from PIL.FontFile import FontFile
from PIL.ImageFont import FreeTypeFont

START_RANGE = 32
END_RANGE = 126


def get_characters_count() -> int:
    return END_RANGE - START_RANGE + 1


def get_max_width_height(font: FreeTypeFont) -> tuple[int, int]:
    max_width = 0
    max_height = 0

    for i in range(START_RANGE, END_RANGE + 1):
        text = chr(i)

        _, _, width, height = font.getbbox(text)
        max_width = max(max_width, width)
        max_height = max(max_height, height)

    return max_width, max_height


def get_bytes_for_char(font: FreeTypeFont, char: str) -> bytes:
    max_width, max_height = get_max_width_height(font)

    image = Image.new("RGB", (int(max_width), int(max_height)), color="white")
    draw = ImageDraw.Draw(image)
    draw.text((0, 0), char, (0, 0, 0), font=font)

    image.save("tmp.ppm")
    data = open("tmp.ppm", "rb").read()
    os.remove("tmp.ppm")

    index = 0
    count = 0

    while count != 3:
        if data[index] == ord("\n"):
            count += 1
        index += 1

    return data[index:]


def generate_font_images(font: FreeTypeFont):
    max_width, max_height = get_max_width_height(font)
    font_name = ".".join(font.path.split(".")[:-1])

    bytes_array = []

    for char in f"P6\n{max_width} {max_height * get_characters_count()}\n255\n":
        bytes_array.append(ord(char))

    for i in range(START_RANGE, END_RANGE + 1):
        bytes_array += get_bytes_for_char(font, chr(i))

    with open(f"{font_name}.ppm", "wb") as file:
        file.write(bytes(bytes_array))

    convert(f"{font_name}.ppm", "png")

def convert(file: str, target_format:str):
    file_without_extension = ".".join(file.split(".")[:-1])
    Image.open(file).save(f"{file_without_extension}.{target_format}")


def main():
    arguments = sys.argv[1:]

    if len(arguments) != 2:
        print("Usage: python fonts.py <font_path> <font_size>")

    font_path = arguments[0]
    font_size = int(arguments[1])

    if os.path.isdir(font_path):
        for real_font_path in os.listdir(font_path):
            if os.path.isdir(f"{font_path}/{real_font_path}"):
                print(f"Skipping directory {font_path}/{real_font_path}")
                continue

            if real_font_path.split(".")[-1] != "ttf":
                print(f"Skipping {font_path}/{real_font_path}")
                continue

            print(f"Generating {font_path}/{real_font_path}")
            font: FreeTypeFont = ImageFont.truetype(f"{font_path}/{real_font_path}", font_size)
            generate_font_images(font)
    else:
        font: FreeTypeFont = ImageFont.truetype(font_path, font_size)
        print(f"Generating {font_path}")
        generate_font_images(font)


if __name__ == "__main__":
    main()
