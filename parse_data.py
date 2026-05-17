import json

file_path = 'response.json'
try:
    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    rows = data.get('rows', [])
    selected_fields = ['stock_id', 'bond_id', 'stock_nm', 'bond_nm', 'progress_nm', 'progress_dt', 'cb_amount', 'price', 'convert_price', 'increase_rt', 'year_left', 'apply10']
    
    for i, row in enumerate(rows[:5]):
        cell = row.get('cell', {})
        print(f"--- Row {i+1} ---")
        for field in selected_fields:
            val = cell.get(field)
            print(f"{field}: {type(val).__name__} = {val}")
except Exception as e:
    print(f"Error: {e}")
