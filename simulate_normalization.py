import json

def apply10_sim(val):
    if isinstance(val, (str, int, float, bool)):
        return str(val)
    return '-'

# Mock data based on the request requirements
mock_data = {
    "rows": [
        {"cell": {"min_market_value": "100.5", "id": "1"}},
        {"cell": {"min_market_value": 200, "id": "2"}},
        {"cell": {"min_market_value": True, "id": "3"}},
        {"cell": {"min_market_value": None, "id": "4"}},
        {"cell": {"min_market_value": ["invalid"], "id": "5"}}
    ]
}

print("Simulating normalization (string/number/bool -> display, others -> '-')")
rows = mock_data.get('rows', [])[:5]
print("min_market_value values:")
for row in rows:
    cell = row.get('cell', {})
    val = cell.get('min_market_value')
    print(apply10_sim(val))
