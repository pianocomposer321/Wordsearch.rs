from wordsearch.app import generate_pdf
from wordsearch.pdf import PDF
import subprocess
import asyncio

WORDS = """cylinder
denial
boot
fossil
compact
nuance
hover
ancestor
asset
disagree
elapse
consumer
have
linen
even
section
fantasy
young
gear
open
"""


async def _main():
    board = await generate_pdf(WORDS)

    pdf = PDF(board, WORDS.split("\n"))
    with open("output.pdf", "wb") as output:
        pdf.save_pdf(output)

    subprocess.run(["xdg-open", "output.pdf"])

def main():
    asyncio.run(_main())
