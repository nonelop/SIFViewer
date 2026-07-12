SIFViewer — a tool for reading files with the ```.sif``` extension, which represent a small image composed of characters and displayed in a terminal window.

The format itself is quite simple: the first 3 bytes store the version and width, while the rest contain the image data. Each "pixel" is allocated 3 bytes — one for each RGB channel.