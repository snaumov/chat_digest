from summa import keywords


def get_keywords(text):
    print(text)
    print(keywords.keywords(text, language='russian'))