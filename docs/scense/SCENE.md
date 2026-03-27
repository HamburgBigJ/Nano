## Scene

### Scene 
```json
{
    "name": "Starting Forest",
    "id": "map_01",
    "entities": [
        {
            "file": "objects/testblock.json",
            "position": { "x": 0.0, "y": 2.0, "z": 0.0 }
        },
        {
            "file": "objects/testblock.json",
            "position": { "x": 10.5, "y": 0.0, "z": -5.0 }
        },
        {
            "file": "objects/hill_slope.json",
            "position": { "x": -5.0, "y": 0.0, "z": 0.0 }
        }
    ]
}
```

### Game objects Spline
```json
{
    "id": "hill_slope",
    "assets": "textures/grass.png",
    "collision": {
        "type": "spline",
        "points": [
            {"x": 0.0, "y": 0.0},
            {"x": 10.0, "y": 5.0},
            {"x": 20.0, "y": 2.0}
        ]
    }
}
```

### Box
```json
{
    "id": "floor_stone",
    "assets": "textures/stone.png",
    "collision": {
        "type": "box",
        "width": 32.0,
        "height": 10.0
    }
}
```