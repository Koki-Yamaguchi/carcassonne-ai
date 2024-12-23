import os
import time

from selenium import webdriver
from selenium.webdriver.support.wait import WebDriverWait
from selenium.webdriver.common.by import By

username = os.environ["BGA_USERNAME"]
password = os.environ["BGA_PASSWORD"]
base_url = os.environ["BGA_BASE_URL"]

def signin(driver):
    driver.get(f'{base_url}/account')

    email_input = WebDriverWait(driver, 10).until(lambda driver: driver.find_element(By.CSS_SELECTOR, "form input[name='email'].text-bga-input-null"))

    email_input.send_keys(username)

    time.sleep(2)

    button = WebDriverWait(driver, 10).until(lambda driver: driver.find_element(By.XPATH, "//a[text()='Next']"))

    button.click()

    password_input = WebDriverWait(driver, 10).until(lambda driver: driver.find_element(By.CSS_SELECTOR, "form input[type='password'].text-bga-input-null"))

    password_input.send_keys(password)

    time.sleep(2)

    button = WebDriverWait(driver, 10).until(lambda driver: driver.find_element(By.XPATH, "//a[text()='Login']"))

    button.click()

    time.sleep(2)

