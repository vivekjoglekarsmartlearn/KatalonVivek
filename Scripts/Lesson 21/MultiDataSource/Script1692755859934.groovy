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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://demowebshop.tricentis.com/')

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop/a_Log in'))

WebUI.setText(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Login/input_Email_Email'), 'ravireddy4qa@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Login/input_Password_Password'), 
    'tCZ2zUMlCYa1Odv8HJk4EA==')

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Login/input_Forgot password_button-1 login-button'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop/a_Apparel  Shoes'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Apparel  Shoes/a_Blue Jeans'))

WebUI.setText(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Blue Jeans/input_Qty_addtocart_36.EnteredQuantity'), 
    '10')

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Blue Jeans/input_Qty_add-to-cart-button-36'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Blue Jeans/span_Shopping cart'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Shopping Cart/input_I agree with the terms of service and_f529a0'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Shopping Cart/button_Checkout'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Checkout/input_Fax number_button-1 new-address-next-_2f210c'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Checkout/input_Fax number_button-1 new-address-next-_2f210c'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Checkout/input__button-1 shipping-method-next-step-button'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Checkout/input__button-1 shipping-method-next-step-button'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Checkout/input__button-1 shipping-method-next-step-button'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Checkout/input__button-1 shipping-method-next-step-button'))

WebUI.click(findTestObject('Object Repository/MultiDataSource/Page_Demo Web Shop. Checkout/a_Log out'))

WebUI.closeBrowser()

