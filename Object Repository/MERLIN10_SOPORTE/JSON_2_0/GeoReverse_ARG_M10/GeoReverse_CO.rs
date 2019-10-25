<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GeoReverse_CO</name>
   <tag></tag>
   <elementGuidId>6a048f27-6d17-42ce-975e-ef1619438915</elementGuidId>
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
   <restUrl>${Geo2Json}?clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;latitude=-34.563153&amp;longitude=-58.464091&amp;level1=AR</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Geo2Json</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>Geo2Json</name>
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
WS.verifyElementPropertyValue(response, 'nGeoReverse.level1', &quot;AR&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.level2', &quot;CAPITAL FEDERAL&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.level3', &quot;CAPITAL FEDERAL&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.level4', &quot;CIUDAD AUTONOMA BUENOS AIRES&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.level5', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.streetType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.street', &quot;AVENIDA DOCTOR RICARDO BALBIN&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.approximateStreetNumber', &quot;2332&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.postalCode', &quot;1428&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.additionalPostalCode', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.initialStreetSegment', 2301)
WS.verifyElementPropertyValue(response, 'nGeoReverse.endStreetSegment', 2400)
WS.verifyElementPropertyValue(response, 'nGeoReverse.betweenStreet1', &quot;AVDA FELIX DE OLAZABAL&quot;)
WS.verifyElementPropertyValue(response, 'nGeoReverse.betweenStreet2', &quot;ALTE MANUEL BLANCO ENCALADA&quot;)
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;zonaFranca&quot;,&quot;value&quot;:&quot;N&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;tipoGeo&quot;,&quot;value&quot;:&quot;1&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nise&quot;,&quot;value&quot;:&quot;9&quot;}')

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
