wget  \
  --method POST \
  --header 'Content-Type: application/json' \
 --body-data '{  "firstname": "Kaia",  "lastname": "Collett",   "electorate": "Perth",   "password": "fjhdbjhdsb",  "dob": {   "day": 26,   "month": "May",   "year": 2005  },   "drivers": {    "number": 34832,  "backnumber": "hsdbjhfbd"   },  "token": "6525150"' \
- http://localhost:8000/register

