import os
import re

def update_links(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Regex to find href="/..." but not href="//..." (protocol relative)
        # and not href="/tower..." (already updated)
        # Capture the path after /
        
        def replace_link(match):
            path = match.group(1)
            if path.startswith('/tower'):
                return match.group(0) # Already correct
            if path.startswith('http'):
                return match.group(0) # External
            
            # Special case for root
            if path == '/':
                return 'href="/tower"'
            
            return f'href="/tower{path}"'

        # Pattern: href="(/[^"]*)"
        new_content = re.sub(r'href="(/[^"]*)"', replace_link, content)
        
        if new_content != content:
            print(f"Updating {filepath}")
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
    except Exception as e:
        print(f"Error processing {filepath}: {e}")

def main():
    target_dir = os.path.join(os.getcwd(), 'apps', 'researcher', 'src')
    
    for root, _, files in os.walk(target_dir):
        for file in files:
            if file.endswith('.rs'):
                update_links(os.path.join(root, file))

if __name__ == "__main__":
    main()
