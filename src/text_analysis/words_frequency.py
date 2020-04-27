import optparse
import os
import nltk
from pymystem3 import Mystem

nltk.download("stopwords")

from nltk.corpus import stopwords
from string import punctuation

# TODO stop list
exclude = u"a"
# TODO min number of occurences
min = 1
# TODO max number of results
length = 100


def get_words_frequency(text):
    print("old", text)
    text = u"".join(ch for ch in text if ch not in punctuation).lower()
    m = Mystem()

    russian_stopwords = ''
    f = open('./russian_stopwords.txt')
    russian_stopwords = f.read().split()
    f.close()

    word_dict = {}
    n = 100000
    text = text.split()
    text = [text[i:i + n] for i in range(0, len(text), n)]

    for chunk in text:
        chunk = " ".join(chunk)
        done = ''.join(m.lemmatize(chunk.encode('utf-8')))

        for word in done.split():
            if word not in word_dict:
                word_dict[word] = {}
                word_dict[word] = 1
            else:
                word_dict[word] += 1


    sorted_word_dict = sorted(word_dict, key=word_dict.get, reverse=True)
    filtered_word_dict = []

    for i, key in enumerate(sorted_word_dict):

        if word_dict[key] >= min:
            if i == length:
                break
            if key in russian_stopwords:
                continue
            filtered_word_dict.append([key, word_dict[key]])
    return filtered_word_dict