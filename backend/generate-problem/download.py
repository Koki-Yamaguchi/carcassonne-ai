import os
import sys
import re
import time
from selenium import webdriver
from selenium.webdriver.common.by import By

from signin import signin

base_url = os.environ["BGA_BASE_URL"]

def download():
    table_id = sys.argv[1]

    driver = webdriver.Chrome()

    signin(driver)

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

