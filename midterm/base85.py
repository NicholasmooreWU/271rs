import base64, sys

s = '<~' + base64.a85encode(open(sys.argv[1], 'rb').read()).decode('ascii') + '~>'

[print(s[i*80:(i+1)*80]) for i in range(len(s)//80)]
print(s[-(len(s)%80):], end='')