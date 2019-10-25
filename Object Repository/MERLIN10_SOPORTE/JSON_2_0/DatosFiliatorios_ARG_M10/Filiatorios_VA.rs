<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Filiatorios_VA</name>
   <tag></tag>
   <elementGuidId>a775268d-afe0-4891-8a8c-cd247639fca7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded; charset=utf-8</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${FiliatoriosJson}?documentNumber=93-28888-5&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.FiliatoriosJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>FiliatoriosJson</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementPropertyValue(response, 'status', &quot;OK&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentType', &quot;96&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentTypeFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentNumber', &quot;93288885&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentNumberFlg', &quot;VA&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryType', &quot;86&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryTypeFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryNumber', &quot;20932888859&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryNumberFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.contributorType', &quot;A&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.contributorTypeFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.lastName', &quot;LAZARO&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.lastNameFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.name', &quot;LUIS&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.nameFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.gender', &quot;M&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.genderFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.birthDate', &quot;15/04/1925&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.birthDateFlg', &quot;AP&quot;)

assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;differenceLevelName&quot;,&quot;value&quot;:&quot;1&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;literalDistance&quot;,&quot;value&quot;:&quot;100.00&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
