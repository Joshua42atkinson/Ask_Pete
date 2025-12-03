import os

def refactor_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        new_content = content.replace('use common', 'use pete_core')
        new_content = new_content.replace('common::', 'pete_core::')
        
        if new_content != content:
            print(f"Updating {filepath}")
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
    except Exception as e:
        print(f"Error processing {filepath}: {e}")

def main():
    dirs_to_process = ['backend', 'frontend', 'apps']
    root_dir = os.getcwd()
    
    for d in dirs_to_process:
        target_dir = os.path.join(root_dir, d)
        if not os.path.exists(target_dir):
            continue
            
        for root, _, files in os.walk(target_dir):
            for file in files:
                if file.endswith('.rs'):
                    refactor_file(os.path.join(root, file))

if __name__ == "__main__":
    main()
