<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BusquedaPredictiva_OK_DTV</name>
   <tag></tag>
   <elementGuidId>69622258-18d0-499c-a634-e363c9150964</elementGuidId>
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
   <restUrl>${Predictiva2Json}?clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;level1=AR&amp;stringSearch=buenos</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Predictiva2Json</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>Predictiva2Json</name>
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



WS.verifyElementPropertyValue(response, 'status', &quot;SD&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)


assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;PEHUAJO&quot;,&quot;level4&quot;:&quot;ABEL&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;6450&quot;,&quot;latitude&quot;:&quot;-35.617993&quot;,&quot;longitude&quot;:&quot;-62.004667&quot;,&quot;label&quot;:&quot;ABEL, PEHUAJO, BUENOS AIRES&quot;,&quot;locationType&quot;:&quot;CITY&quot;,&quot;maps&quot;:false,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-35.617993&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-62.004667&quot;}]}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;AZUL&quot;,&quot;level4&quot;:&quot;AZUL&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;7300&quot;,&quot;latitude&quot;:&quot;-36.777488&quot;,&quot;longitude&quot;:&quot;-59.863553&quot;,&quot;label&quot;:&quot;AZUL, AZUL, BUENOS AIRES&quot;,&quot;locationType&quot;:&quot;CITY&quot;,&quot;maps&quot;:true,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-36.780092&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-59.865415&quot;}]}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;AYACUCHO&quot;,&quot;level4&quot;:&quot;FAIR&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;7153&quot;,&quot;latitude&quot;:&quot;-37.066156&quot;,&quot;longitude&quot;:&quot;-58.302124&quot;,&quot;label&quot;:&quot;FAIR, AYACUCHO, BUENOS AIRES&quot;,&quot;locationType&quot;:&quot;CITY&quot;,&quot;maps&quot;:false,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-37.066156&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-58.302124&quot;}]}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;CORONEL DORREGO&quot;,&quot;level4&quot;:&quot;GIL&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;8151&quot;,&quot;latitude&quot;:&quot;-38.792084&quot;,&quot;longitude&quot;:&quot;-60.909036&quot;,&quot;label&quot;:&quot;GIL, CORONEL DORREGO, BUENOS AIRES&quot;,&quot;locationType&quot;:&quot;CITY&quot;,&quot;maps&quot;:false,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-38.792084&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-60.909036&quot;}]}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;GENERAL ARENALES&quot;,&quot;level4&quot;:&quot;HAM&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;6005&quot;,&quot;latitude&quot;:&quot;-34.226145&quot;,&quot;longitude&quot;:&quot;-61.244489&quot;,&quot;label&quot;:&quot;HAM, GENERAL ARENALES, BUENOS AIRES&quot;,&quot;locationType&quot;:&quot;CITY&quot;,&quot;maps&quot;:false,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-34.226145&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-61.244489&quot;}]}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;ALBERTI&quot;,&quot;level4&quot;:&quot;PLA&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;6634&quot;,&quot;latitude&quot;:&quot;-35.126449&quot;,&quot;longitude&quot;:&quot;-60.219876&quot;,&quot;label&quot;:&quot;PLA, ALBERTI, BUENOS AIRES&quot;,&quot;locationType&quot;:&quot;CITY&quot;,&quot;maps&quot;:false,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-35.126449&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-60.219876&quot;}]}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;CHASCOMUS&quot;,&quot;level4&quot;:&quot;ADELA&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;7136&quot;,&quot;latitude&quot;:&quot;-35.684877&quot;,&quot;longitude&quot;:&quot;-57.956156&quot;,&quot;label&quot;:&quot;ADELA, CHASCOMUS, BUENOS AIRES&quot;,&quot;locationType&quot;:&quot;CITY&quot;,&quot;maps&quot;:false,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-35.684877&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-57.956156&quot;}]}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;MERCEDES&quot;,&quot;level4&quot;:&quot;AGOTE&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;6608&quot;,&quot;latitude&quot;:&quot;-34.645016&quot;,&quot;longitude&quot;:&quot;-59.359276&quot;,&quot;label&quot;:&quot;AGOTE, MERCEDES, BUENOS AIRES&quot;,&quot;locationType&quot;:&quot;CITY&quot;,&quot;maps&quot;:false,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-34.645016&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-59.359276&quot;}]}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;GUAMINI&quot;,&quot;level4&quot;:&quot;ALAMOS&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;6437&quot;,&quot;latitude&quot;:&quot;-36.894234&quot;,&quot;longitude&quot;:&quot;-62.356312&quot;,&quot;label&quot;:&quot;ALAMOS, GUAMINI, BUENOS AIRES&quot;,&quot;locationType&quot;:&quot;CITY&quot;,&quot;maps&quot;:false,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-36.894234&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-62.356312&quot;}]}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;CORDOBA&quot;,&quot;level3&quot;:&quot;RIO CUARTO&quot;,&quot;level4&quot;:&quot;BULNES&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;&quot;,&quot;street&quot;:&quot;BUENOS AIRES&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;postalCode&quot;:&quot;5845&quot;,&quot;latitude&quot;:&quot;-33.505395&quot;,&quot;longitude&quot;:&quot;-64.675322&quot;,&quot;label&quot;:&quot;BUENOS AIRES, BULNES, RIO CUARTO, CORDOBA&quot;,&quot;locationType&quot;:&quot;STREET&quot;,&quot;maps&quot;:false,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;level4latitudeDTV&quot;,&quot;value&quot;:&quot;-33.505395&quot;},{&quot;name&quot;:&quot;level4longitudeDTV&quot;,&quot;value&quot;:&quot;-64.675322&quot;}]}]}')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
