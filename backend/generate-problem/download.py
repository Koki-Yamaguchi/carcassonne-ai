import os
import sys
import re
import time
from selenium import webdriver
from selenium.webdriver.chrome.service import Service as ChromeService
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.common.by import By

username = os.environ["BGA_USERNAME"]
password = os.environ["BGA_PASSWORD"]
base_url = os.environ["BGA_BASE_URL"]

def download():
    table_id = sys.argv[1]

    # driver = webdriver.Chrome()
    driver = webdriver.Chrome(service=ChromeService(ChromeDriverManager().install()))

    driver.get(f'{base_url}/account')
    time.sleep(1)

    user_input = driver.find_element(By.ID, 'username_input')
    user_input.send_keys(username)
    user_pass = driver.find_element(By.ID, 'password_input')
    user_pass.send_keys(password)
    time.sleep(1)

    submit_button = driver.find_element(By.ID, 'submit_login_button')
    submit_button.click()
    time.sleep(3)

    driver.get(f'{base_url}/gamereview?table={table_id}')
    time.sleep(1)

    choose_player_button = driver.find_element(By.CLASS_NAME, 'choosePlayerLink')
    choose_player_button.click()
    time.sleep(3)

    html = driver.page_source

    moves = re.match('.*g_gamelogs = (.*?);\n.*', html, re.S).group(1)

    moves = re.match('.*g_gamelogs = (.*?);\n.*', html, re.S).group(1)

    f = open(f'./../src/data/{table_id}.json', "w")
    f.write(moves)
    f.close()

    driver.quit()

if __name__ == '__main__':
    download()

