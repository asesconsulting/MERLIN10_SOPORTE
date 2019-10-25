<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_MAQ_Telefonica</name>
   <tag></tag>
   <elementGuidId>78f6a5b8-297c-4065-a1b0-b035b7a95e93</elementGuidId>
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
   <restUrl>${AddressJson}?level1=AR&amp;street=APECECHEA%2087&amp;level4=BURZACO&amp;clientAccessCode=f35c5a599b268a499da6e4cae7f7a265</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.AddressJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>AddressJson</name>
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


WS.verifyElementPropertyValue(response, 'status', &quot;CO&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.geoType', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level1', &quot;AR&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level2', &quot;BUENOS AIRES&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level3', &quot;ALMIRANTE BROWN&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level4', &quot;BURZACO&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level5', &quot;LAS ROSAS&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.streetType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.street', &quot;APECECHEA&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.houseNumber', &quot;87&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.floor', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.unit', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.latitude', &quot;-34.827127&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.longitude', &quot;-58.370503&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.postalCode', &quot;1852&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalPostalCode', &quot;B1852ICA&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.fromStreetNumber', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.toStreetNumber', &quot;1200&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalData', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet1', &quot;PARANA&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet2', &quot;AVDA MONTEVERDE&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.corner', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.place', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeReference', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.numberAlternativeAddresses', &quot;0&quot;)
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;puerta&quot;,&quot;value&quot;:&quot;SI&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;latitudLocalidad&quot;,&quot;value&quot;:&quot;-34.829406&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;zonaRiesgo&quot;,&quot;value&quot;:&quot;N&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;NISE&quot;,&quot;value&quot;:&quot;3&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;longitudLocalidad&quot;,&quot;value&quot;:&quot;-58.402093&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Latitude&quot;,&quot;value&quot;:&quot;-34.829406&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nise&quot;,&quot;value&quot;:&quot;3&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Longitude&quot;,&quot;value&quot;:&quot;-58.402093&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;inMaq&quot;,&quot;value&quot;:&quot;0&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqValue&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqConcept&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;merlinRiskArea&quot;,&quot;value&quot;:&quot;N&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqStatus&quot;,&quot;value&quot;:&quot;NO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nivelRiesgo&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;cpa&quot;,&quot;value&quot;:&quot;B1852ICA&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;tipoVivienda&quot;,&quot;value&quot;:&quot;otro&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;concepto&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;postalCertifiedAddresses&quot;,&quot;value&quot;:&quot;SI&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;empresa&quot;,&quot;value&quot;:&quot;NO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;zonaFranca&quot;,&quot;value&quot;:&quot;N&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
