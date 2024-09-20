import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//Start Scripting
//Open Browser
WebUI.openBrowser('https://example.testproject.io/web/')

WebUI.maximizeWindow()

//Set Text in userName
WebUI.setText(findTestObject('Object Repository/TC-03 Scripting Mode/Page_TestProject Demo/input_Full Name_name'), 'Ravikanth')

//Set Text in Password
WebUI.setText(findTestObject('Object Repository/TC-03 Scripting Mode/Page_TestProject Demo/input_Password_password'), '12345')

//Click on login button
WebUI.click(findTestObject('Object Repository/TC-03 Scripting Mode/Page_TestProject Demo/button_Login'))

//validate the home page
not_run: WebUI.verifyElementPresent(findTestObject('Object Repository/TC-03 Scripting Mode/Page_TestProject Demo/h1_TestProject Example page'), 
    4)

//Click on logout
WebUI.click(findTestObject('Object Repository/TC-03 Scripting Mode/Page_TestProject Demo/button_Logout'))

//Close the Browser
WebUI.closeBrowser()

