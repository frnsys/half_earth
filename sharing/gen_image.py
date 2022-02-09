import os
import random
import textwrap
from PIL import Image, ImageFont, ImageDraw, ImageOps

msgs_lose = [
    "were ousted from power by a furious mob in {year}",
    "were exiled by revolutionaries from the Global South in {year}",
    "were assassinated by a coalition from the Global South in {year}",
    "were executed by a reactionary force formed by the former wealthy nations in {year}"
]
msgs_win = [
    "ushered the world into a prosperous future",
]

prefixes = [
    'Try your best',
    'Give it a shot',
    'Take a chance',
    'Better the world',
    'Don\'t mess up',
    'Have a go',
    'Turn things around',
]

bgs_win = [os.path.join('images/win', f) for f in os.listdir('images/win')]
bgs_lose = [os.path.join('images/lose', f) for f in os.listdir('images/lose')]

# for Twitter
size = (1200, 675)
badge_size = 52
badge_spacing = 8
v_padding = 10
lg_font = ImageFont.truetype('fonts/TimesTen.ttf', 86)
sm_font = ImageFont.truetype('fonts/Inter-Medium.ttf', 42)
badge_font = ImageFont.truetype('fonts/Inter-Light.ttf', 18)

def msg_from_summary(summary):
    # TODO
    if summary['win']:
        return random.choice(msgs_win)
    else:
        return random.choice(msgs_lose)


def gen_image(year, summary, outpath):
    badges = [
        Image.open(
            os.path.join('badges', '{}.png'.format(badge)))
            .resize((badge_size, badge_size))
        for badge in summary['badges']
    ]

    if summary['win']:
        bg = random.choice(bgs_win)
    else:
        bg = random.choice(bgs_lose)

    thumbnail = ImageOps.fit(
        Image.open(bg),
        size,
        Image.ANTIALIAS
    )

    msg = msg_from_summary(summary)
    text = 'Me and {}s {}.'.format(
            summary['faction'], msg.format(year=year))
    para = textwrap.wrap(text, width=26)

    img = Image.new('RGB', size, color=0)
    img.paste(thumbnail)

    draw = ImageDraw.Draw(img)


    badges_width = len(badges) * badge_size + (len(badges) - 1) * badge_spacing
    x = round(size[0]/2 - badges_width/2)
    by = 16

    r_padding = 15
    rx = x-r_padding
    ry = by - r_padding
    draw.rounded_rectangle((rx, ry, rx+badges_width+r_padding, ry+badge_size + 2*r_padding), fill="#20202000", radius=16)

    for f in badges:
        img.paste(f, box=(x, by), mask=f)
        # draw.text((x, 16+badge_size), 'Nuclear',
        #         font=badge_font, fill='#000', stroke_width=2, stroke_fill='#fff')
        x += badge_size + badge_spacing

    _, line_height = draw.textsize(text, font=lg_font)
    n_lines = len(para)
    height = n_lines * line_height + (n_lines - 1) * v_padding
    y = size[1]/2 - height/2 - line_height/2 + (badge_size + 2*r_padding)/2

    for line in para:
        w, h = draw.textsize(line, font=lg_font)
        draw.text(((size[0] - w) / 2, y), line,
                font=lg_font, stroke_width=3, stroke_fill='#000')
        y += h + v_padding

    text = '{} at half.earth'.format(random.choice(prefixes))
    w, h = draw.textsize(text, font=sm_font)
    draw.text(((size[0] - w) / 2, size[1] - h - 16), text,
            font=sm_font, fill='#FEC007', stroke_width=2, stroke_fill='#000')

    img.save(outpath, quality=95)
