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

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_CUIL'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_DNI_DU'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_DNI'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_CIIU'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_HS'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_ITAU'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_Identidad'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_Contactabilidad'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_SituacionEnconomica'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_SituacionLaboral'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_OtrasVariables'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_2_0/Enrichment _ARGM10/Enrichment_Crediticia'))

