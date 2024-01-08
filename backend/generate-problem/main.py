import os
import sys
import re
import time
import chromedriver_binary
from selenium import webdriver
from selenium.webdriver.common.by import By
import requests
from get_chrome_driver import GetChromeDriver

username = os.environ["BGA_USERNAME"]
password = os.environ["BGA_PASSWORD"]
base_url = os.environ["BGA_BASE_URL"]
api_base_url = os.environ["API_BASE_URL"]

def get_problem_proposals():
    headers = {'Content-Type': 'application/json'}
    res = requests.get(f'{api_base_url}/problem-proposals', headers=headers)
    res.raise_for_status()
    proposals = res.json()
    return proposals

def create_problem(creator_id, remaining_tile_count, moves, note):
    headers = {'Content-Type': 'application/json'}
    data = {
        'creator_id': creator_id,
        'remaining_tile_count': remaining_tile_count,
        'moves': moves,
        'note': note,
    }
    print(data)
    res = requests.post(f'{api_base_url}/problems/create', headers=headers, json=data)
    res.raise_for_status()
    problem = res.json()
    return problem

def use_proposal(id):
    headers = {'Content-Type': 'application/json'}
    res = requests.post(f'{api_base_url}/problem-proposals/{id}/use', headers=headers)
    res.raise_for_status()
    proposal = res.json()
    return proposal

def run():
    proposals = get_problem_proposals()
    if len(proposals) == 0:
        return

    # local
    # driver = webdriver.Chrome()

    # remote
    get_driver = GetChromeDriver()
    get_driver.install()
    options = webdriver.ChromeOptions()
    options.add_argument('--headless')
    driver = webdriver.Chrome(options=options)

    print("found proposals")
    print(proposals)
    signed_in = False
    for proposal in proposals:
        table_id = proposal['table_id']

        if not signed_in:
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

            signed_in = True

        driver.get(f'{base_url}/gamereview?table={table_id}')
        time.sleep(1)

        choose_player_button = driver.find_element(By.CLASS_NAME, 'choosePlayerLink')
        choose_player_button.click()
        time.sleep(3)

        html = driver.page_source

        moves = re.match('.*g_gamelogs = (.*?);\n.*', html, re.S).group(1)

        id = proposal['id']
        creator_id = proposal['creator_id']
        remaining_tile_count = proposal['remaining_tile_count']
        note = proposal['note']

        problem = create_problem(creator_id, remaining_tile_count, moves, note)
        print(problem)

        proposal = use_proposal(id)
        print(proposal)

    driver.quit()

if __name__ == '__main__':
    run()

