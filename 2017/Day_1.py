captcha = open('Day_1_sateinput').read()[:-2]
captcha_chars = list(map(int, list(captcha)))
print(captcha_chars)
sum_of_captcha_1 = 0
for i in range(1, len(captcha_chars)):
    sum_of_captcha_1 += captcha_chars[i-1] if captcha_chars[i-1] == captcha_chars[i] else 0
sum_of_captcha_1 += captcha_chars[-1] if captcha_chars[-1] == captcha_chars[0] else 0

sum_of_captcha_2 = 0
half_length = int(len(captcha_chars)/2)
for i in range(half_length):
    sum_of_captcha_2 += captcha_chars[i] if captcha_chars[i] == captcha_chars[i+half_length] else 0
    sum_of_captcha_2 += captcha_chars[i+half_length] if captcha_chars[i+half_length] == captcha_chars[i] else 0
print(sum_of_captcha_1, sum_of_captcha_2)
