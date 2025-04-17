# query Archidekt's api using pyrchidekt

from pyrchidekt.api import getDeckById

# define the web address of the desired deck
deck_address = "https://archidekt.com/decks/7783724/ovika_burgers"

deck = getDeckById(7783724)
for card in deck.cards:
    print(card.card.artist)
