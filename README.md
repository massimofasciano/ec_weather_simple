## ec_weather_simple

Get current weather conditions from Environment Canada at this site:
https://dd.weather.gc.ca/citypage_weather/xml/

Data is parsed from XML and displayed as a JSON object containing the current conditions.

This is a simplified version of https://github.com/massimofasciano/ec_weather,
without any significant error checking or argument parsing.
It simply converts the "currentConditions" XML node into JSON.

The full URL for the XML can be passed as a single argument. 
It defaults to a specific weather station otherwise.

```
$ ec_weather_simple https://dd.weather.gc.ca/citypage_weather/xml/ON/s0000073_e.xml
{
   "temperature" : {
      "unitType" : "metric",
      "units" : "C",
      "value" : 13.9
   },
   "relativeHumidity" : {
      "value" : 58,
      "units" : "%"
   },
   "pressure" : {
      "value" : 101.1,
      "tendency" : "falling",
      "units" : "kPa",
      "change" : 0,
      "unitType" : "metric"
   },
   "visibility" : {
      "unitType" : "metric",
      "units" : "km"
   },
   "wind" : {
      "direction" : "ESE",
      "gust" : {
         "unitType" : "metric",
         "units" : "km/h"
      },
      "speed" : {
         "units" : "km/h",
         "unitType" : "metric",
         "value" : 2
      },
      "bearing" : {
         "value" : 107.9,
         "units" : "degrees"
      }
   },
   "dewpoint" : {
      "value" : 5.8,
      "units" : "C",
      "unitType" : "metric"
   },
   "windChill" : {
      "value" : -1,
      "unitType" : "metric"
   },
   "station" : {
      "value" : "Algonquin Park East Gate",
      "code" : "tnk",
      "lon" : "78.27W",
      "lat" : "45.53N"
   },
   "dateTime" : [
      {
         "textSummary" : "Saturday April 15, 2023 at 23:00 UTC",
         "UTCOffset" : 0,
         "name" : "observation",
         "minute" : "00",
         "year" : 2023,
         "zone" : "UTC",
         "timeStamp" : 20230415230000,
         "month" : {
            "name" : "April",
            "value" : "04"
         },
         "hour" : 23,
         "day" : {
            "value" : 15,
            "name" : "Saturday"
         }
      },
      {
         "minute" : "00",
         "year" : 2023,
         "UTCOffset" : -5,
         "textSummary" : "Saturday April 15, 2023 at 18:00 EST",
         "name" : "observation",
         "timeStamp" : 20230415180000,
         "day" : {
            "name" : "Saturday",
            "value" : 15
         },
         "hour" : 18,
         "month" : {
            "value" : "04",
            "name" : "April"
         },
         "zone" : "EST"
      }
   ],
   "iconCode" : {
      "format" : "gif"
   }
}

$ ec_weather_simple https://dd.weather.gc.ca/citypage_weather/xml/ON/s0000873_e.xml
{
   "error" : "Failed to fetch weather data for https://dd.weather.gc.ca/citypage_weather/xml/ON/s0000873_e.xml with status: 404 Not Found"
}
```

This simplified version is meant to be used as a plugin for Home Assistant, which can easily parse JSON, so there
is no need to extract any of the values. Just outputting the appropriate part of the XML as JSON is enough.
The fancy command line parsing is also irrelevant since the URL is handled by Home Assistant.

It is rougly equivalent to the following piece of Python code:

```python
import json
import xmltodict
import requests
import sys

ec_url = "https://dd.weather.gc.ca/citypage_weather/xml/QC/s0000635_e.xml"
if len(sys.argv) > 1:
    ec_url = sys.argv[1]

data_dict = xmltodict.parse(requests.get(ec_url,stream=True).content,cdata_key="value",attr_prefix='')

json_data = json.dumps(data_dict["siteData"]["currentConditions"])

print(json_data)
```