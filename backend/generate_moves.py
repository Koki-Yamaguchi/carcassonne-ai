import os
import sys
import re
import time
import chromedriver_binary
from selenium import webdriver
from selenium.webdriver.common.by import By
import requests

def get_problem_proposals():
    headers = {'Content-Type': 'application/json'}
    res = requests.get('http://0.0.0.0:8000/problem-proposals', headers=headers)
    proposals = res.json()
    return proposals

def main():
    proposals = get_problem_proposals()
    print(proposals)
    if len(proposals) == 0:
        return

    driver = webdriver.Chrome()

    for proposal in proposals:
        table_id = proposal['table_id']
        username = os.environ["BGA_USERNAME"]
        password = os.environ["BGA_PASSWORD"]
        base_url = os.environ["BGA_BASE_URL"]

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

        # TODO: save moves in postgres

        # TODO: create draft problem

    driver.quit()

if __name__ == '__main__':
    main()

