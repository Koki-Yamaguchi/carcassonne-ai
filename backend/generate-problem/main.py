import os
import sys
import re
import time
import chromedriver_binary
from selenium import webdriver
from selenium.webdriver.common.by import By
import requests
from get_chrome_driver import GetChromeDriver

def get_problem_proposals():
    headers = {'Content-Type': 'application/json'}
    res = requests.get('http://0.0.0.0:8000/problem-proposals', headers=headers)
    proposals = res.json()
    return proposals

def create_problem(creator_id, remaining_tile_count, moves):
    headers = {'Content-Type': 'application/json'}
    data = {
        'creator_id': creator_id,
        'remaining_tile_count': remaining_tile_count,
        'moves': moves,
    }
    res = requests.post('http://0.0.0.0:8000/problems/create', headers=headers, json=data)
    problem = res.json()
    return problem

def main():
    proposals = get_problem_proposals()
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

        creator_id = proposal['creator_id']
        remaining_tile_count = proposal['remaining_tile_count']
        problem = create_problem(creator_id, remaining_tile_count, moves)

    driver.quit()

def test():
    # os.environ['WDM_SSL_VERIFY'] = '0'

    get_driver = GetChromeDriver()
    get_driver.install()

    options = webdriver.ChromeOptions()
    options.add_argument('--headless')

    driver = webdriver.Chrome(options=options)

    table_id = "447137094"

    username = os.environ["BGA_USERNAME"]
    password = os.environ["BGA_PASSWORD"]
    base_url = "https://boardgamearena.com"

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
    print(moves)

    driver.quit()


if __name__ == '__main__':
    # main()
    test()

