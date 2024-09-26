import os
import sys

from PIL import Image


def convert(file: str, target_format: str):
    file_without_extension = ".".join(file.split(".")[:-1])
    Image.open(file).convert("RGB").save(f"{file_without_extension}.{target_format}")


def main():
    arguments = sys.argv[1:]

    if len(arguments) != 2:
        print("Usage: python fonts.py <image_path> <target_format>")

    image_path = arguments[0]
    target_format = arguments[1]

    print(f"Attempting to convert {image_path} to {target_format}")

    if os.path.isdir(image_path):
        for real_image_path in os.listdir(image_path):
            if os.path.isdir(f"{image_path}/{real_image_path}"):
                print(f"Skipping directory {image_path}/{real_image_path}")
                continue

            if real_image_path.split(".")[-1] == target_format:
                print(f"Skipping {image_path}/{real_image_path}")
                continue

            print(f"Converting {image_path}/{real_image_path}")
            convert(f"{image_path}/{real_image_path}", target_format)
    else:
        print(f"Converting {image_path}")
        convert(image_path, target_format)


if __name__ == "__main__":
    main()