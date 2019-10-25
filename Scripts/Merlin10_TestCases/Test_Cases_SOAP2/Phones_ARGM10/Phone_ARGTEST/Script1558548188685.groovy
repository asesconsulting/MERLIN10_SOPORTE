import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Phones_ARGM10/Phones_Celular_CO'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Phones_ARGM10/Phones_Celular_CO_Nollame'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Phones_ARGM10/Phones_Fijo_CO'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Phones_ARGM10/Phones_Fijo_CO_NoLlame'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Phones_ARGM10/Phones_Macro_CO'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Phones_ARGM10/Phones_NE'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Phones_ARGM10/Phones_SD'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Phones_ARGM10/Phones_TN_CO'))

