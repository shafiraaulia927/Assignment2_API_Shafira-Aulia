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

//testcase method GET
response = WS.sendRequest(findTestObject('Photos/GET-Photos'))
WS.verifyResponseStatusCode(response, 200)
WS.verifyElementPropertyValue(response, '[2].albumId', '1')
WS.verifyElementPropertyValue(response, '[2].id', '3')
WS.verifyElementPropertyValue(response, '[2].title', 'officia porro iure quia iusto qui ipsa ut modi')
WS.verifyElementPropertyValue(response, '[2].url', 'https://via.placeholder.com/600/24f355')
WS.verifyElementPropertyValue(response, '[2].thumbnailUrl', 'https://via.placeholder.com/150/24f355')

//testcase method POST
response = WS.sendRequest(findTestObject('Photos/POST-Photos'))
WS.verifyResponseStatusCode(response, 201)
WS.verifyElementPropertyValue(response, 'albumId', '1')
WS.verifyElementPropertyValue(response, 'id', '5001')
WS.verifyElementPropertyValue(response, 'title', 'asignment 2')
WS.verifyElementPropertyValue(response, 'url', 'https://via.placeholder.com/600/92c952')
WS.verifyElementPropertyValue(response, 'thumbnailUrl', 'https://via.placeholder.com/150/92c952')

//testcase method DELETE
response = WS.sendRequest(findTestObject('Photos/DELETE-Photos'))
WS.verifyResponseStatusCode(response, 200)