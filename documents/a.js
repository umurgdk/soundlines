FROM PHONE:

{
  location: {
    lat: 1,
      lng: 2
  },

    soundLevel: 0.4,
    ....
}

TO PHONE AS AGGREGATED ALSO FROM OTHERS:
{
  locations: [
    // 5 around
    { lat, lng }
    { lat, lng }
    { lat, lng }
    { lat, lng }
    { lat, lng }
  ],

  soundLevel: 0.65 // avarage,
  light: 0.34
}

TO PHONE AS FOREST DATA (INITIAL DATA):
{
  regions: [
    {
      vegitation: 0.75,
      animals: [
        { ... },
        { ... },
        { ... },
        { ... },
      ]
      trees: [
        { length: 3, lat: 13213, lng: 234234  },
        { length: 3, lat: 13213, lng: 234234  },
        { length: 3, lat: 13213, lng: 234234  },
        { length: 3, lat: 13213, lng: 234234  },
        { length: 3, lat: 13213, lng: 234234  },
      ],
    }
  ]
}

AFTER DATA:
{
  diff: [
    { type: "tree", id: 1231223, length: 0.1 },
    { type: "tree", id: 1231226, length: -0.3 },
    { type: "noise", id: 5, level: -0.04 },
    { type: "noise", id: 8, level: 0.04 },
  ]
}
