import os
import random
import textwrap
from PIL import Image, ImageFont, ImageDraw, ImageOps

msgs_lose = [
    "were ousted from power by a furious mob in {year}",
    "were exiled by popular revolutionaries {year}",
    "were assassinated by a global coalition in {year}",
]
msgs_win = [
    "ushered the world into a prosperous future",
]
msgs_died = [
    "failed to make the world a better place in time",
]
msgs_pc = [
    "were pushed out of parliament in a coup in {year}",
    "were executed by a reactionary force formed by the former wealthy nations in {year}",
]

bgs_win = [os.path.join('images/win', f) for f in os.listdir('images/win')]
bgs_lose = [os.path.join('images/lose/generic', f) for f in os.listdir('images/lose/generic')]
bgs_death = [os.path.join('images/lose/death', f) for f in os.listdir('images/lose/death')]
bgs_coup = [os.path.join('images/lose/coup', f) for f in os.listdir('images/lose/coup')]

# for Twitter
size = (1200, 675)
badge_size = 52
badge_spacing = 8
v_padding = 10
lg_font = ImageFont.truetype('fonts/TimesTen.ttf', 86)
sm_font = ImageFont.truetype('fonts/Inter-Medium.ttf', 42)
badge_font = ImageFont.truetype('fonts/Inter-Light.ttf', 18)


def msg_from_summary(summary):
    if summary['win']:
        return random.choice(msgs_win)
    else:
        s = summary['scenario']
        if s['political_capital'] <= 0:
            return random.choice(msgs_pc)
        elif s['world']['year'] >= s['death_year']:
            return random.choice(msgs_died)
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
        s = summary['scenario']
        if s['political_capital'] <= 0:
            bg = random.choice(bgs_coup)
        elif s['world']['year'] >= s['death_year']:
            bg = random.choice(bgs_death)
        else:
            bg = random.choice(bgs_lose)


    thumbnail = ImageOps.fit(
        Image.open(bg),
        size,
        Image.ANTIALIAS
    )

    msg = msg_from_summary(summary)
    fac = summary['faction']
    ending = 's' if not fac.endswith('s') else ''
    text = 'Me and {}{} {}.'.format(fac, ending, msg.format(year=year))
    para = textwrap.wrap(text, width=26)

    img = Image.new('RGB', size, color=0)
    img.paste(thumbnail)

    draw = ImageDraw.Draw(img)

    badges_width = len(badges) * badge_size + (len(badges) - 1) * badge_spacing
    x = round(size[0]/2 - badges_width/2)
    by = 16

    r_padding = 15
    # rx = x - r_padding
    # ry = by - r_padding
    # draw.rounded_rectangle((rx, ry, rx+badges_width+r_padding, ry+badge_size + 2*r_padding), fill="#20202000", radius=16)

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

    text = 'Play at half.earth'
    w, h = draw.textsize(text, font=sm_font)
    draw.text(((size[0] - w) / 2, size[1] - h - 16), text,
            font=sm_font, fill='#FEC007', stroke_width=2, stroke_fill='#000')

    img.save(outpath, quality=15)
