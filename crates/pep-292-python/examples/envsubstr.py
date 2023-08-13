from pep_292_python import Template 
from sys import stdin
from os import environ

template = Template(stdin.read())
res = template.substitute(**environ)
print(res, end='')
